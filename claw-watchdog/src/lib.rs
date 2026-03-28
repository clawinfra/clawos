// claw-watchdog: Health monitor and watchdog for ClawOS agents
// Phase 4 — PID liveness, MQTT heartbeat timeout, auto-restart, anomaly detection.
// Currently a stub; will be implemented in Phase 4.

/// Watchdog configuration.
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// How often to check agent health (seconds).
    pub check_interval_secs: u64,
    /// Heartbeat timeout before marking agent unhealthy (seconds).
    pub heartbeat_timeout_secs: u64,
    /// Whether to automatically restart failed agents.
    pub auto_restart: bool,
    /// Maximum restarts before giving up (crash-loop detection).
    pub max_restarts: u32,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            check_interval_secs: 10,
            heartbeat_timeout_secs: 30,
            auto_restart: true,
            max_restarts: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_watchdog_config() {
        let config = WatchdogConfig::default();
        assert_eq!(config.check_interval_secs, 10);
        assert_eq!(config.heartbeat_timeout_secs, 30);
        assert!(config.auto_restart);
        assert_eq!(config.max_restarts, 5);
    }
}
