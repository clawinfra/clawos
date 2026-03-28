use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a managed agent process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    /// Agent is starting up.
    Starting,
    /// Agent is running and healthy.
    Running,
    /// Agent has been stopped gracefully.
    Stopped,
    /// Agent process exited unexpectedly.
    Failed,
    /// Agent is being restarted.
    Restarting,
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Stopped => write!(f, "stopped"),
            Self::Failed => write!(f, "failed"),
            Self::Restarting => write!(f, "restarting"),
        }
    }
}

/// Configuration for spawning an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Human-readable name for the agent.
    pub name: String,
    /// Path to the agent binary.
    pub binary_path: String,
    /// Command-line arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables.
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    /// Optional DID for on-chain identity.
    #[serde(default)]
    pub did: Option<String>,
}

/// A registered agent with its runtime state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEntry {
    /// Unique identifier (UUIDv7 for time-ordering).
    pub id: Uuid,
    /// Agent configuration.
    pub config: AgentConfig,
    /// Current status.
    pub status: AgentStatus,
    /// OS process ID (if running).
    pub pid: Option<u32>,
    /// Number of times this agent has been restarted.
    pub restart_count: u32,
    /// When the agent was first registered.
    pub created_at: DateTime<Utc>,
    /// When the status last changed.
    pub updated_at: DateTime<Utc>,
    /// Exit code from the last run (if exited).
    pub last_exit_code: Option<i32>,
}

impl AgentEntry {
    /// Create a new agent entry from config.
    pub fn new(config: AgentConfig) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            config,
            status: AgentStatus::Starting,
            pid: None,
            restart_count: 0,
            created_at: now,
            updated_at: now,
            last_exit_code: None,
        }
    }

    /// Update the status and refresh the updated_at timestamp.
    pub fn set_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_status_display() {
        assert_eq!(AgentStatus::Starting.to_string(), "starting");
        assert_eq!(AgentStatus::Running.to_string(), "running");
        assert_eq!(AgentStatus::Stopped.to_string(), "stopped");
        assert_eq!(AgentStatus::Failed.to_string(), "failed");
        assert_eq!(AgentStatus::Restarting.to_string(), "restarting");
    }

    #[test]
    fn test_agent_status_serialize() {
        let status = AgentStatus::Running;
        let json = serde_json::to_string(&status).expect("serialize");
        assert_eq!(json, "\"running\"");
    }

    #[test]
    fn test_agent_status_deserialize() {
        let status: AgentStatus = serde_json::from_str("\"failed\"").expect("deserialize");
        assert_eq!(status, AgentStatus::Failed);
    }

    #[test]
    fn test_agent_config_defaults() {
        let json = r#"{"name":"test","binary_path":"/bin/echo"}"#;
        let config: AgentConfig = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.name, "test");
        assert!(config.args.is_empty());
        assert!(config.env.is_empty());
        assert!(config.did.is_none());
    }

    #[test]
    fn test_agent_entry_new() {
        let config = AgentConfig {
            name: "test-agent".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec!["hello".to_string()],
            env: std::collections::HashMap::new(),
            did: None,
        };
        let entry = AgentEntry::new(config);
        assert_eq!(entry.status, AgentStatus::Starting);
        assert!(entry.pid.is_none());
        assert_eq!(entry.restart_count, 0);
        assert!(entry.last_exit_code.is_none());
    }

    #[test]
    fn test_agent_entry_set_status() {
        let config = AgentConfig {
            name: "test-agent".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec![],
            env: std::collections::HashMap::new(),
            did: None,
        };
        let mut entry = AgentEntry::new(config);
        let before = entry.updated_at;
        // Small sleep to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(2));
        entry.set_status(AgentStatus::Running);
        assert_eq!(entry.status, AgentStatus::Running);
        assert!(entry.updated_at >= before);
    }

    #[test]
    fn test_agent_entry_serialization_roundtrip() {
        let config = AgentConfig {
            name: "roundtrip-agent".to_string(),
            binary_path: "/usr/bin/test".to_string(),
            args: vec!["--flag".to_string()],
            env: [("KEY".to_string(), "VALUE".to_string())]
                .into_iter()
                .collect(),
            did: Some("did:claw:abc123".to_string()),
        };
        let entry = AgentEntry::new(config);
        let json = serde_json::to_string(&entry).expect("serialize");
        let restored: AgentEntry = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(entry.id, restored.id);
        assert_eq!(entry.config.name, restored.config.name);
        assert_eq!(entry.status, restored.status);
    }
}
