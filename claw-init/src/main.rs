mod config;
mod handlers;
mod server;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use tokio::signal;
use tracing::{info, warn};

use claw_agent_registry::AgentRegistry;
use config::ClawdConfig;
use handlers::AppState;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let config_path = std::env::var("CLAWD_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/etc/clawd/config.toml"));

    let home_config = dirs_or_default().join("config.toml");
    let config = if config_path.exists() {
        ClawdConfig::load(&config_path)?
    } else if home_config.exists() {
        ClawdConfig::load(&home_config)?
    } else {
        info!("no config file found, using defaults");
        ClawdConfig::default()
    };

    // Initialize tracing
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.clawd.log_level));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    info!(version = VERSION, "clawd starting");

    // Build app state
    let registry = AgentRegistry::new();
    let state = Arc::new(AppState::new(registry));

    // Build router
    let app = server::build_router(state);

    // Bind and serve
    let addr = config.http_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!(addr = %addr, "HTTP API listening");

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("claw-init stopped");
    Ok(())
}

/// Wait for SIGTERM or SIGINT for graceful shutdown.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            warn!("received Ctrl+C, shutting down");
        }
        () = terminate => {
            warn!("received SIGTERM, shutting down");
        }
    }
}

/// Get the default clawd data directory (~/.clawd/).
fn dirs_or_default() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".clawd")
}
