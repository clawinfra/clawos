// claw-mqtt: MQTT broker integration for ClawOS
// Phase 2 — embedded rumqttd broker, topic definitions, client helpers.
// Currently a stub; will be implemented in Phase 2.

/// MQTT topic namespace for ClawOS.
pub mod topics {
    /// Agent status topic: `claw/agents/{id}/status`
    pub fn agent_status(agent_id: &str) -> String {
        format!("claw/agents/{agent_id}/status")
    }

    /// Agent task topic: `claw/agents/{id}/task`
    pub fn agent_task(agent_id: &str) -> String {
        format!("claw/agents/{agent_id}/task")
    }

    /// Agent metrics topic: `claw/agents/{id}/metrics`
    pub fn agent_metrics(agent_id: &str) -> String {
        format!("claw/agents/{agent_id}/metrics")
    }

    /// System health topic.
    pub const SYSTEM_HEALTH: &str = "claw/system/health";

    /// System events topic.
    pub const SYSTEM_EVENTS: &str = "claw/system/events";
}

#[cfg(test)]
mod tests {
    use super::topics;

    #[test]
    fn test_topic_formats() {
        assert_eq!(
            topics::agent_status("abc-123"),
            "claw/agents/abc-123/status"
        );
        assert_eq!(topics::agent_task("abc-123"), "claw/agents/abc-123/task");
        assert_eq!(
            topics::agent_metrics("abc-123"),
            "claw/agents/abc-123/metrics"
        );
        assert_eq!(topics::SYSTEM_HEALTH, "claw/system/health");
    }
}
