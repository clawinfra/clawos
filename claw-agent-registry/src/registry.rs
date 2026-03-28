use std::sync::Arc;

use dashmap::DashMap;
use tracing::{info, warn};
use uuid::Uuid;

use crate::models::{AgentConfig, AgentEntry, AgentStatus};
use crate::spawner::ProcessSpawner;

/// Thread-safe agent registry backed by DashMap.
#[derive(Clone)]
pub struct AgentRegistry {
    agents: Arc<DashMap<Uuid, AgentEntry>>,
    spawner: Arc<ProcessSpawner>,
}

impl AgentRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            agents: Arc::new(DashMap::new()),
            spawner: Arc::new(ProcessSpawner::new()),
        }
    }

    /// Create a registry with a custom spawner (useful for testing).
    pub fn with_spawner(spawner: ProcessSpawner) -> Self {
        Self {
            agents: Arc::new(DashMap::new()),
            spawner: Arc::new(spawner),
        }
    }

    /// Spawn a new agent from the given config.
    /// Returns the agent entry on success.
    pub async fn spawn_agent(&self, config: AgentConfig) -> anyhow::Result<AgentEntry> {
        let mut entry = AgentEntry::new(config);
        let id = entry.id;

        info!(agent_id = %id, name = %entry.config.name, "spawning agent");

        match self.spawner.spawn(&entry.config).await {
            Ok(pid) => {
                entry.pid = Some(pid);
                entry.set_status(AgentStatus::Running);
                info!(agent_id = %id, pid = pid, "agent spawned successfully");
            }
            Err(e) => {
                warn!(agent_id = %id, error = %e, "failed to spawn agent");
                entry.set_status(AgentStatus::Failed);
                self.agents.insert(id, entry.clone());
                return Err(e);
            }
        }

        self.agents.insert(id, entry.clone());
        Ok(entry)
    }

    /// List all registered agents.
    pub fn list_agents(&self) -> Vec<AgentEntry> {
        self.agents
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get a single agent by ID.
    pub fn get_agent(&self, id: &Uuid) -> Option<AgentEntry> {
        self.agents.get(id).map(|entry| entry.value().clone())
    }

    /// Stop and deregister an agent.
    /// Returns the agent entry if found.
    pub async fn stop_agent(&self, id: &Uuid) -> anyhow::Result<Option<AgentEntry>> {
        let entry = match self.agents.get(id) {
            Some(entry) => entry.value().clone(),
            None => return Ok(None),
        };

        if let Some(pid) = entry.pid {
            info!(agent_id = %id, pid = pid, "stopping agent");
            self.spawner.kill(pid).await?;
        }

        // Remove from registry and update status
        if let Some((_, mut removed)) = self.agents.remove(id) {
            removed.set_status(AgentStatus::Stopped);
            info!(agent_id = %id, "agent stopped and deregistered");
            Ok(Some(removed))
        } else {
            Ok(None)
        }
    }

    /// Restart an agent: stop it, then spawn a new process with the same config.
    /// Returns the updated entry.
    pub async fn restart_agent(&self, id: &Uuid) -> anyhow::Result<Option<AgentEntry>> {
        let entry = match self.agents.get(id) {
            Some(entry) => entry.value().clone(),
            None => return Ok(None),
        };

        // Kill existing process if any
        if let Some(pid) = entry.pid {
            info!(agent_id = %id, pid = pid, "killing agent for restart");
            self.spawner.kill(pid).await?;
        }

        // Update status to restarting
        if let Some(mut agent) = self.agents.get_mut(id) {
            agent.set_status(AgentStatus::Restarting);
        }

        // Spawn new process
        match self.spawner.spawn(&entry.config).await {
            Ok(pid) => {
                if let Some(mut agent) = self.agents.get_mut(id) {
                    agent.pid = Some(pid);
                    agent.restart_count += 1;
                    agent.set_status(AgentStatus::Running);
                    info!(agent_id = %id, pid = pid, restarts = agent.restart_count, "agent restarted");
                    Ok(Some(agent.clone()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                warn!(agent_id = %id, error = %e, "failed to restart agent");
                if let Some(mut agent) = self.agents.get_mut(id) {
                    agent.set_status(AgentStatus::Failed);
                    agent.restart_count += 1;
                }
                Err(e)
            }
        }
    }

    /// Get the total number of registered agents.
    pub fn count(&self) -> usize {
        self.agents.len()
    }

    /// Count agents with a specific status.
    pub fn count_by_status(&self, status: AgentStatus) -> usize {
        self.agents
            .iter()
            .filter(|entry| entry.value().status == status)
            .count()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spawner::SpawnMode;

    fn test_config(name: &str) -> AgentConfig {
        AgentConfig {
            name: name.to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec!["hello".to_string()],
            env: std::collections::HashMap::new(),
            did: None,
        }
    }

    fn mock_registry() -> AgentRegistry {
        AgentRegistry::with_spawner(ProcessSpawner::with_mode(SpawnMode::Mock))
    }

    #[tokio::test]
    async fn test_spawn_agent() {
        let registry = mock_registry();
        let entry = registry
            .spawn_agent(test_config("agent-1"))
            .await
            .expect("spawn should succeed");
        assert_eq!(entry.config.name, "agent-1");
        assert_eq!(entry.status, AgentStatus::Running);
        assert!(entry.pid.is_some());
        assert_eq!(registry.count(), 1);
    }

    #[tokio::test]
    async fn test_list_agents() {
        let registry = mock_registry();
        registry
            .spawn_agent(test_config("a"))
            .await
            .expect("spawn");
        registry
            .spawn_agent(test_config("b"))
            .await
            .expect("spawn");
        registry
            .spawn_agent(test_config("c"))
            .await
            .expect("spawn");

        let agents = registry.list_agents();
        assert_eq!(agents.len(), 3);
    }

    #[tokio::test]
    async fn test_get_agent() {
        let registry = mock_registry();
        let entry = registry
            .spawn_agent(test_config("find-me"))
            .await
            .expect("spawn");

        let found = registry.get_agent(&entry.id);
        assert!(found.is_some());
        assert_eq!(found.as_ref().expect("found").config.name, "find-me");

        let not_found = registry.get_agent(&Uuid::now_v7());
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_stop_agent() {
        let registry = mock_registry();
        let entry = registry
            .spawn_agent(test_config("stop-me"))
            .await
            .expect("spawn");

        let stopped = registry
            .stop_agent(&entry.id)
            .await
            .expect("stop should succeed");
        assert!(stopped.is_some());
        assert_eq!(stopped.as_ref().expect("stopped").status, AgentStatus::Stopped);

        // Agent should be removed from registry
        assert_eq!(registry.count(), 0);
        assert!(registry.get_agent(&entry.id).is_none());
    }

    #[tokio::test]
    async fn test_stop_nonexistent_agent() {
        let registry = mock_registry();
        let result = registry
            .stop_agent(&Uuid::now_v7())
            .await
            .expect("should not error");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_restart_agent() {
        let registry = mock_registry();
        let entry = registry
            .spawn_agent(test_config("restart-me"))
            .await
            .expect("spawn");

        let restarted = registry
            .restart_agent(&entry.id)
            .await
            .expect("restart should succeed");
        assert!(restarted.is_some());
        let restarted = restarted.expect("restarted");
        assert_eq!(restarted.status, AgentStatus::Running);
        assert_eq!(restarted.restart_count, 1);
        assert!(restarted.pid.is_some());
    }

    #[tokio::test]
    async fn test_restart_nonexistent_agent() {
        let registry = mock_registry();
        let result = registry
            .restart_agent(&Uuid::now_v7())
            .await
            .expect("should not error");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_count_by_status() {
        let registry = mock_registry();
        registry
            .spawn_agent(test_config("a"))
            .await
            .expect("spawn");
        registry
            .spawn_agent(test_config("b"))
            .await
            .expect("spawn");

        assert_eq!(registry.count_by_status(AgentStatus::Running), 2);
        assert_eq!(registry.count_by_status(AgentStatus::Failed), 0);
        assert_eq!(registry.count_by_status(AgentStatus::Stopped), 0);
    }

    #[tokio::test]
    async fn test_multiple_restarts_increment_count() {
        let registry = mock_registry();
        let entry = registry
            .spawn_agent(test_config("multi-restart"))
            .await
            .expect("spawn");

        for i in 1..=3 {
            let restarted = registry
                .restart_agent(&entry.id)
                .await
                .expect("restart")
                .expect("found");
            assert_eq!(restarted.restart_count, i);
        }
    }
}
