use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers::{self, AppState};

/// Build the axum router with all ClawOS HTTP API routes.
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // Agent management
        .route("/agents", get(handlers::list_agents))
        .route("/agents/spawn", post(handlers::spawn_agent))
        .route("/agents/:id", get(handlers::get_agent))
        .route("/agents/:id", delete(handlers::delete_agent))
        .route("/agents/:id/restart", post(handlers::restart_agent))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        // State
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use claw_agent_registry::registry::AgentRegistry;
    use claw_agent_registry::spawner::{ProcessSpawner, SpawnMode};

    fn test_state() -> Arc<AppState> {
        let registry =
            AgentRegistry::with_spawner(ProcessSpawner::with_mode(SpawnMode::Mock));
        Arc::new(AppState::new(registry))
    }

    #[test]
    fn test_build_router() {
        // Just verify it doesn't panic
        let _router = build_router(test_state());
    }
}
