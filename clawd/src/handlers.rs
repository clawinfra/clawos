use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::error;

use claw_agent_registry::{AgentConfig, AgentEntry, AgentRegistry, AgentStatus};

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub registry: AgentRegistry,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub version: &'static str,
}

impl AppState {
    pub fn new(registry: AgentRegistry) -> Self {
        Self {
            registry,
            start_time: chrono::Utc::now(),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

/// GET /health — system health check.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: String,
    pub uptime_secs: i64,
    pub agents_total: usize,
    pub agents_running: usize,
    pub agents_failed: usize,
}

pub async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let uptime = chrono::Utc::now() - state.start_time;
    Json(HealthResponse {
        status: "ok",
        version: state.version.to_string(),
        uptime_secs: uptime.num_seconds(),
        agents_total: state.registry.count(),
        agents_running: state.registry.count_by_status(AgentStatus::Running),
        agents_failed: state.registry.count_by_status(AgentStatus::Failed),
    })
}

/// GET /agents — list all agents.
#[derive(Serialize)]
pub struct AgentsListResponse {
    pub agents: Vec<AgentEntry>,
    pub total: usize,
}

pub async fn list_agents(State(state): State<Arc<AppState>>) -> Json<AgentsListResponse> {
    let agents = state.registry.list_agents();
    let total = agents.len();
    Json(AgentsListResponse { agents, total })
}

/// POST /agents/spawn — spawn a new agent.
#[derive(Deserialize)]
pub struct SpawnRequest {
    pub name: String,
    pub binary_path: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub did: Option<String>,
}

impl From<SpawnRequest> for AgentConfig {
    fn from(req: SpawnRequest) -> Self {
        Self {
            name: req.name,
            binary_path: req.binary_path,
            args: req.args,
            env: req.env,
            did: req.did,
        }
    }
}

#[derive(Serialize)]
pub struct SpawnResponse {
    pub agent: AgentEntry,
}

pub async fn spawn_agent(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SpawnRequest>,
) -> Result<(StatusCode, Json<SpawnResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Validate required fields
    if req.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "name is required".to_string(),
            }),
        ));
    }
    if req.binary_path.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "binary_path is required".to_string(),
            }),
        ));
    }

    let config: AgentConfig = req.into();
    match state.registry.spawn_agent(config).await {
        Ok(agent) => Ok((StatusCode::CREATED, Json(SpawnResponse { agent }))),
        Err(e) => {
            error!(error = %e, "failed to spawn agent");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("failed to spawn agent: {e}"),
                }),
            ))
        }
    }
}

/// GET /agents/:id — get agent details.
pub async fn get_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<AgentEntry>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = parse_uuid(&id)?;
    match state.registry.get_agent(&uuid) {
        Some(agent) => Ok(Json(agent)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("agent {id} not found"),
            }),
        )),
    }
}

/// DELETE /agents/:id — stop and deregister an agent.
#[derive(Serialize)]
pub struct DeleteResponse {
    pub message: String,
    pub agent: AgentEntry,
}

pub async fn delete_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = parse_uuid(&id)?;
    match state.registry.stop_agent(&uuid).await {
        Ok(Some(agent)) => Ok(Json(DeleteResponse {
            message: format!("agent {} stopped and deregistered", id),
            agent,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("agent {id} not found"),
            }),
        )),
        Err(e) => {
            error!(error = %e, "failed to stop agent");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("failed to stop agent: {e}"),
                }),
            ))
        }
    }
}

/// POST /agents/:id/restart — restart an agent.
#[derive(Serialize)]
pub struct RestartResponse {
    pub message: String,
    pub agent: AgentEntry,
}

pub async fn restart_agent(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<RestartResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = parse_uuid(&id)?;
    match state.registry.restart_agent(&uuid).await {
        Ok(Some(agent)) => Ok(Json(RestartResponse {
            message: format!("agent {} restarted", id),
            agent,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("agent {id} not found"),
            }),
        )),
        Err(e) => {
            error!(error = %e, "failed to restart agent");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("failed to restart agent: {e}"),
                }),
            ))
        }
    }
}

/// Standard error response.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Parse a UUID from a string path parameter.
fn parse_uuid(s: &str) -> Result<uuid::Uuid, (StatusCode, Json<ErrorResponse>)> {
    s.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("invalid UUID: {s}"),
            }),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_request_to_config() {
        let req = SpawnRequest {
            name: "test".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec!["hello".to_string()],
            env: std::collections::HashMap::new(),
            did: Some("did:claw:abc".to_string()),
        };
        let config: AgentConfig = req.into();
        assert_eq!(config.name, "test");
        assert_eq!(config.binary_path, "/bin/echo");
        assert_eq!(config.did, Some("did:claw:abc".to_string()));
    }

    #[test]
    fn test_parse_uuid_valid() {
        let uuid = uuid::Uuid::now_v7();
        let result = parse_uuid(&uuid.to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_invalid() {
        let result = parse_uuid("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_response_serialization() {
        let err = ErrorResponse {
            error: "something went wrong".to_string(),
        };
        let json = serde_json::to_string(&err).expect("serialize");
        assert!(json.contains("something went wrong"));
    }
}
