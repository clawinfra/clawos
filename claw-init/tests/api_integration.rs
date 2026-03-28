use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use tower::ServiceExt;

use claw_agent_registry::registry::AgentRegistry;
use claw_agent_registry::spawner::{ProcessSpawner, SpawnMode};
use claw_agent_registry::{AgentEntry, AgentStatus};

// ============================================================================
// Test app state and handlers (duplicated from binary crate for integration testing)
// ============================================================================

#[derive(Clone)]
struct TestAppState {
    registry: AgentRegistry,
    start_time: chrono::DateTime<chrono::Utc>,
}

impl TestAppState {
    fn new(registry: AgentRegistry) -> Self {
        Self {
            registry,
            start_time: chrono::Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct HealthResp {
    status: String,
    version: String,
    uptime_secs: i64,
    agents_total: usize,
    agents_running: usize,
    agents_failed: usize,
}

#[derive(Serialize, Deserialize)]
struct AgentsListResp {
    agents: Vec<AgentEntry>,
    total: usize,
}

#[derive(Deserialize)]
struct SpawnReq {
    name: String,
    binary_path: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: std::collections::HashMap<String, String>,
    #[serde(default)]
    did: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SpawnResp {
    agent: AgentEntry,
}

#[derive(Serialize, Deserialize)]
struct DeleteResp {
    message: String,
    agent: AgentEntry,
}

#[derive(Serialize, Deserialize)]
struct RestartResp {
    message: String,
    agent: AgentEntry,
}

#[derive(Serialize, Deserialize)]
struct ErrorResp {
    error: String,
}

// Handler functions

async fn handle_health(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
) -> axum::Json<HealthResp> {
    let uptime = chrono::Utc::now() - state.start_time;
    axum::Json(HealthResp {
        status: "ok".to_string(),
        version: "0.1.0-test".to_string(),
        uptime_secs: uptime.num_seconds(),
        agents_total: state.registry.count(),
        agents_running: state.registry.count_by_status(AgentStatus::Running),
        agents_failed: state.registry.count_by_status(AgentStatus::Failed),
    })
}

async fn handle_list_agents(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
) -> axum::Json<AgentsListResp> {
    let agents = state.registry.list_agents();
    let total = agents.len();
    axum::Json(AgentsListResp { agents, total })
}

async fn handle_spawn_agent(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
    axum::Json(req): axum::Json<SpawnReq>,
) -> Result<(StatusCode, axum::Json<SpawnResp>), (StatusCode, axum::Json<ErrorResp>)> {
    if req.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResp {
                error: "name is required".to_string(),
            }),
        ));
    }
    if req.binary_path.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResp {
                error: "binary_path is required".to_string(),
            }),
        ));
    }

    let config = claw_agent_registry::AgentConfig {
        name: req.name,
        binary_path: req.binary_path,
        args: req.args,
        env: req.env,
        did: req.did,
    };

    match state.registry.spawn_agent(config).await {
        Ok(agent) => Ok((StatusCode::CREATED, axum::Json(SpawnResp { agent }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResp {
                error: format!("{e}"),
            }),
        )),
    }
}

async fn handle_get_agent(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::Json<AgentEntry>, (StatusCode, axum::Json<ErrorResp>)> {
    let uuid: uuid::Uuid = id.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResp {
                error: format!("invalid UUID: {id}"),
            }),
        )
    })?;
    match state.registry.get_agent(&uuid) {
        Some(agent) => Ok(axum::Json(agent)),
        None => Err((
            StatusCode::NOT_FOUND,
            axum::Json(ErrorResp {
                error: format!("agent {id} not found"),
            }),
        )),
    }
}

async fn handle_delete_agent(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::Json<DeleteResp>, (StatusCode, axum::Json<ErrorResp>)> {
    let uuid: uuid::Uuid = id.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResp {
                error: format!("invalid UUID: {id}"),
            }),
        )
    })?;
    match state.registry.stop_agent(&uuid).await {
        Ok(Some(agent)) => Ok(axum::Json(DeleteResp {
            message: format!("agent {id} stopped"),
            agent,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            axum::Json(ErrorResp {
                error: format!("agent {id} not found"),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResp {
                error: format!("{e}"),
            }),
        )),
    }
}

async fn handle_restart_agent(
    axum::extract::State(state): axum::extract::State<Arc<TestAppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::Json<RestartResp>, (StatusCode, axum::Json<ErrorResp>)> {
    let uuid: uuid::Uuid = id.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResp {
                error: format!("invalid UUID: {id}"),
            }),
        )
    })?;
    match state.registry.restart_agent(&uuid).await {
        Ok(Some(agent)) => Ok(axum::Json(RestartResp {
            message: format!("agent {id} restarted"),
            agent,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            axum::Json(ErrorResp {
                error: format!("agent {id} not found"),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorResp {
                error: format!("{e}"),
            }),
        )),
    }
}

// Router builder

fn test_router() -> axum::Router {
    let registry = AgentRegistry::with_spawner(ProcessSpawner::with_mode(SpawnMode::Mock));
    let state = Arc::new(TestAppState::new(registry));

    axum::Router::new()
        .route("/health", axum::routing::get(handle_health))
        .route("/agents", axum::routing::get(handle_list_agents))
        .route("/agents/spawn", axum::routing::post(handle_spawn_agent))
        .route("/agents/:id", axum::routing::get(handle_get_agent))
        .route("/agents/:id", axum::routing::delete(handle_delete_agent))
        .route(
            "/agents/:id/restart",
            axum::routing::post(handle_restart_agent),
        )
        .with_state(state)
}

// Helper

async fn body_json<T: serde::de::DeserializeOwned>(body: Body) -> T {
    let bytes = body.collect().await.expect("collect body").to_bytes();
    serde_json::from_slice(&bytes).expect("parse JSON")
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_health_endpoint() {
    let app = test_router();
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::OK);

    let body: HealthResp = body_json(response.into_body()).await;
    assert_eq!(body.status, "ok");
    assert_eq!(body.agents_total, 0);
    assert_eq!(body.agents_running, 0);
    assert_eq!(body.agents_failed, 0);
}

#[tokio::test]
async fn test_list_agents_empty() {
    let app = test_router();
    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::OK);

    let body: AgentsListResp = body_json(response.into_body()).await;
    assert_eq!(body.total, 0);
    assert!(body.agents.is_empty());
}

#[tokio::test]
async fn test_spawn_and_list_agent() {
    let app = test_router();

    // Spawn an agent
    let spawn_body = serde_json::json!({
        "name": "test-agent",
        "binary_path": "/bin/echo",
        "args": ["hello"]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::CREATED);

    let body: SpawnResp = body_json(response.into_body()).await;
    assert_eq!(body.agent.config.name, "test-agent");
    assert_eq!(body.agent.status, AgentStatus::Running);
    let agent_id = body.agent.id;

    // List should now have one agent
    let req = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    let body: AgentsListResp = body_json(response.into_body()).await;
    assert_eq!(body.total, 1);

    // Get specific agent
    let req = Request::builder()
        .uri(&format!("/agents/{agent_id}"))
        .body(Body::empty())
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::OK);

    let body: AgentEntry = body_json(response.into_body()).await;
    assert_eq!(body.config.name, "test-agent");
}

#[tokio::test]
async fn test_spawn_validation_empty_name() {
    let app = test_router();
    let spawn_body = serde_json::json!({
        "name": "",
        "binary_path": "/bin/echo"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_spawn_validation_empty_binary() {
    let app = test_router();
    let spawn_body = serde_json::json!({
        "name": "test",
        "binary_path": ""
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_agent_not_found() {
    let app = test_router();
    let fake_id = uuid::Uuid::now_v7();

    let req = Request::builder()
        .uri(&format!("/agents/{fake_id}"))
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_agent_invalid_uuid() {
    let app = test_router();

    let req = Request::builder()
        .uri("/agents/not-a-uuid")
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_agent_lifecycle() {
    let app = test_router();

    // Spawn first
    let spawn_body = serde_json::json!({
        "name": "delete-me",
        "binary_path": "/bin/echo"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    let body: SpawnResp = body_json(response.into_body()).await;
    let agent_id = body.agent.id;

    // Delete it
    let req = Request::builder()
        .method("DELETE")
        .uri(&format!("/agents/{agent_id}"))
        .body(Body::empty())
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::OK);

    let body: DeleteResp = body_json(response.into_body()).await;
    assert_eq!(body.agent.status, AgentStatus::Stopped);

    // Should be gone now
    let req = Request::builder()
        .uri(&format!("/agents/{agent_id}"))
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_agent() {
    let app = test_router();
    let fake_id = uuid::Uuid::now_v7();

    let req = Request::builder()
        .method("DELETE")
        .uri(&format!("/agents/{fake_id}"))
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_restart_agent_lifecycle() {
    let app = test_router();

    // Spawn first
    let spawn_body = serde_json::json!({
        "name": "restart-me",
        "binary_path": "/bin/echo"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    let body: SpawnResp = body_json(response.into_body()).await;
    let agent_id = body.agent.id;

    // Restart it
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{agent_id}/restart"))
        .body(Body::empty())
        .expect("build request");

    let response = app.clone().oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::OK);

    let body: RestartResp = body_json(response.into_body()).await;
    assert_eq!(body.agent.status, AgentStatus::Running);
    assert_eq!(body.agent.restart_count, 1);
}

#[tokio::test]
async fn test_restart_nonexistent_agent() {
    let app = test_router();
    let fake_id = uuid::Uuid::now_v7();

    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{fake_id}/restart"))
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health_reflects_agent_counts() {
    let app = test_router();

    // Spawn two agents
    for name in ["agent-a", "agent-b"] {
        let spawn_body = serde_json::json!({
            "name": name,
            "binary_path": "/bin/echo"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/agents/spawn")
            .header("content-type", "application/json")
            .body(Body::from(spawn_body.to_string()))
            .expect("build request");
        let response = app.clone().oneshot(req).await.expect("request");
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    // Health should show 2 running
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    let body: HealthResp = body_json(response.into_body()).await;
    assert_eq!(body.agents_total, 2);
    assert_eq!(body.agents_running, 2);
    assert_eq!(body.agents_failed, 0);
}

#[tokio::test]
async fn test_full_lifecycle() {
    let app = test_router();

    // 1. Health check — empty
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    let h: HealthResp = body_json(resp.into_body()).await;
    assert_eq!(h.agents_total, 0);

    // 2. Spawn agent
    let spawn_body = serde_json::json!({
        "name": "lifecycle-test",
        "binary_path": "/bin/sleep",
        "args": ["1000"],
        "did": "did:claw:lifecycle"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::CREATED);
    let s: SpawnResp = body_json(resp.into_body()).await;
    let id = s.agent.id;

    // 3. Get agent
    let req = Request::builder()
        .uri(&format!("/agents/{id}"))
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    // 4. Restart agent
    let req = Request::builder()
        .method("POST")
        .uri(&format!("/agents/{id}/restart"))
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);
    let r: RestartResp = body_json(resp.into_body()).await;
    assert_eq!(r.agent.restart_count, 1);

    // 5. Delete agent
    let req = Request::builder()
        .method("DELETE")
        .uri(&format!("/agents/{id}"))
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::OK);

    // 6. Verify gone
    let req = Request::builder()
        .uri(&format!("/agents/{id}"))
        .body(Body::empty())
        .expect("request");
    let resp = app.clone().oneshot(req).await.expect("response");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // 7. Health — back to zero
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .expect("request");
    let resp = app.oneshot(req).await.expect("response");
    let h: HealthResp = body_json(resp.into_body()).await;
    assert_eq!(h.agents_total, 0);
}

#[tokio::test]
async fn test_spawn_with_env_and_did() {
    let app = test_router();

    let spawn_body = serde_json::json!({
        "name": "env-agent",
        "binary_path": "/bin/echo",
        "args": ["--verbose"],
        "env": {"RUST_LOG": "debug", "MY_VAR": "value"},
        "did": "did:claw:test123"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/agents/spawn")
        .header("content-type", "application/json")
        .body(Body::from(spawn_body.to_string()))
        .expect("build request");

    let response = app.oneshot(req).await.expect("request");
    assert_eq!(response.status(), StatusCode::CREATED);

    let body: SpawnResp = body_json(response.into_body()).await;
    assert_eq!(body.agent.config.name, "env-agent");
    assert_eq!(body.agent.config.did, Some("did:claw:test123".to_string()));
    assert_eq!(body.agent.config.env.len(), 2);
    assert_eq!(body.agent.config.args, vec!["--verbose"]);
}
