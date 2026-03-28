use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::{Context, Result};
use tracing::{debug, info};

use crate::models::AgentConfig;

/// Controls whether the spawner uses real processes or mock PIDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnMode {
    /// Spawn real OS processes via tokio::process::Command.
    Real,
    /// Return mock PIDs (for testing without side effects).
    Mock,
}

/// Manages spawning and killing agent processes.
pub struct ProcessSpawner {
    mode: SpawnMode,
    /// Counter for mock PIDs.
    mock_pid: AtomicU32,
}

impl ProcessSpawner {
    /// Create a spawner in real mode.
    pub fn new() -> Self {
        Self {
            mode: SpawnMode::Real,
            mock_pid: AtomicU32::new(10000),
        }
    }

    /// Create a spawner with a specific mode.
    pub fn with_mode(mode: SpawnMode) -> Self {
        Self {
            mode,
            mock_pid: AtomicU32::new(10000),
        }
    }

    /// Spawn a new process for the agent config.
    /// Returns the process ID.
    pub async fn spawn(&self, config: &AgentConfig) -> Result<u32> {
        match self.mode {
            SpawnMode::Real => self.spawn_real(config).await,
            SpawnMode::Mock => self.spawn_mock(config),
        }
    }

    /// Kill a process by PID.
    pub async fn kill(&self, pid: u32) -> Result<()> {
        match self.mode {
            SpawnMode::Real => self.kill_real(pid).await,
            SpawnMode::Mock => {
                debug!(pid = pid, "mock: killed process");
                Ok(())
            }
        }
    }

    async fn spawn_real(&self, config: &AgentConfig) -> Result<u32> {
        let mut cmd = tokio::process::Command::new(&config.binary_path);
        cmd.args(&config.args);

        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        // Detach stdio so the child doesn't inherit our terminal
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());

        let child = cmd
            .spawn()
            .with_context(|| format!("failed to spawn {}", config.binary_path))?;

        let pid = child.id().context("child process has no PID")?;
        info!(
            name = %config.name,
            binary = %config.binary_path,
            pid = pid,
            "spawned real process"
        );

        Ok(pid)
    }

    fn spawn_mock(&self, config: &AgentConfig) -> Result<u32> {
        let pid = self.mock_pid.fetch_add(1, Ordering::SeqCst);
        debug!(
            name = %config.name,
            pid = pid,
            "mock: spawned process"
        );
        Ok(pid)
    }

    async fn kill_real(&self, pid: u32) -> Result<()> {
        // Send SIGTERM first
        let pid_i32 =
            i32::try_from(pid).with_context(|| format!("PID {pid} too large for signal"))?;

        // Safety: we're sending a signal to a known PID
        let result = unsafe { libc::kill(pid_i32, libc::SIGTERM) };
        if result == 0 {
            info!(pid = pid, "sent SIGTERM to process");
        } else {
            let err = std::io::Error::last_os_error();
            // ESRCH = process doesn't exist (already dead) — not an error
            if err.raw_os_error() == Some(libc::ESRCH) {
                debug!(pid = pid, "process already exited");
            } else {
                return Err(err).with_context(|| format!("failed to kill PID {pid}"));
            }
        }

        Ok(())
    }
}

impl Default for ProcessSpawner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_spawn() {
        let spawner = ProcessSpawner::with_mode(SpawnMode::Mock);
        let config = AgentConfig {
            name: "test".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec![],
            env: std::collections::HashMap::new(),
            did: None,
        };

        let pid1 = spawner.spawn(&config).await.expect("spawn");
        let pid2 = spawner.spawn(&config).await.expect("spawn");
        assert_ne!(pid1, pid2);
        assert!(pid1 >= 10000);
    }

    #[tokio::test]
    async fn test_mock_kill() {
        let spawner = ProcessSpawner::with_mode(SpawnMode::Mock);
        spawner.kill(12345).await.expect("mock kill should succeed");
    }

    #[tokio::test]
    async fn test_real_spawn_echo() {
        let spawner = ProcessSpawner::new();
        let config = AgentConfig {
            name: "echo-test".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec!["hello".to_string()],
            env: std::collections::HashMap::new(),
            did: None,
        };

        let pid = spawner.spawn(&config).await.expect("echo should spawn");
        assert!(pid > 0);
    }

    #[tokio::test]
    async fn test_real_spawn_nonexistent_binary() {
        let spawner = ProcessSpawner::new();
        let config = AgentConfig {
            name: "bad".to_string(),
            binary_path: "/nonexistent/binary/path".to_string(),
            args: vec![],
            env: std::collections::HashMap::new(),
            did: None,
        };

        let result = spawner.spawn(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_real_kill_nonexistent_pid() {
        let spawner = ProcessSpawner::new();
        // PID 999999 almost certainly doesn't exist
        let result = spawner.kill(999999).await;
        // Should succeed (process already dead = not an error)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_spawn_with_env() {
        let spawner = ProcessSpawner::with_mode(SpawnMode::Mock);
        let config = AgentConfig {
            name: "env-test".to_string(),
            binary_path: "/bin/echo".to_string(),
            args: vec![],
            env: [("MY_VAR".to_string(), "my_value".to_string())]
                .into_iter()
                .collect(),
            did: Some("did:claw:test123".to_string()),
        };

        let pid = spawner.spawn(&config).await.expect("spawn with env");
        assert!(pid >= 10000);
    }
}
