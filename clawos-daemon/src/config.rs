use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Top-level clawos configuration, loaded from TOML.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClawdConfig {
    #[serde(default)]
    pub clawos: DaemonConfig,
    #[serde(default)]
    pub http: HttpConfig,
    #[serde(default)]
    pub mqtt: MqttConfig,
    #[serde(default)]
    pub chain: ChainConfig,
    #[serde(default)]
    pub watchdog: WatchdogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            data_dir: default_data_dir(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            bind: default_bind(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            port: default_mqtt_port(),
            max_connections: default_max_connections(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    #[serde(default = "default_true")]
    pub dry_run: bool,
    #[serde(default = "default_rpc_url")]
    pub rpc_url: String,
    #[serde(default = "default_key_path")]
    pub key_path: String,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            dry_run: true,
            rpc_url: default_rpc_url(),
            key_path: default_key_path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogConfig {
    #[serde(default = "default_check_interval")]
    pub check_interval_secs: u64,
    #[serde(default = "default_heartbeat_timeout")]
    pub heartbeat_timeout_secs: u64,
    #[serde(default = "default_true")]
    pub auto_restart: bool,
    #[serde(default = "default_max_restarts")]
    pub max_restarts: u32,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            check_interval_secs: default_check_interval(),
            heartbeat_timeout_secs: default_heartbeat_timeout(),
            auto_restart: true,
            max_restarts: default_max_restarts(),
        }
    }
}

// Default value functions
fn default_log_level() -> String {
    "info".to_string()
}
fn default_data_dir() -> String {
    "~/.clawos/data".to_string()
}
fn default_bind() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    7070
}
fn default_mqtt_port() -> u16 {
    1883
}
fn default_max_connections() -> u32 {
    100
}
fn default_true() -> bool {
    true
}
fn default_rpc_url() -> String {
    "ws://127.0.0.1:9944".to_string()
}
fn default_key_path() -> String {
    "~/.clawos/keys/agent.key".to_string()
}
fn default_check_interval() -> u64 {
    10
}
fn default_heartbeat_timeout() -> u64 {
    30
}
fn default_max_restarts() -> u32 {
    5
}

impl ClawdConfig {
    /// Load config from a TOML file, falling back to defaults for missing fields.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read config from {}", path.display()))?;
        let config: Self =
            toml::from_str(&content).with_context(|| "failed to parse TOML config")?;
        Ok(config)
    }

    /// Load from path if it exists, otherwise return defaults.
    #[allow(dead_code)]
    pub fn load_or_default(path: &Path) -> Self {
        Self::load(path).unwrap_or_default()
    }

    /// Get the socket address string for the HTTP server.
    pub fn http_addr(&self) -> String {
        format!("{}:{}", self.http.bind, self.http.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClawdConfig::default();
        assert_eq!(config.http.port, 7070);
        assert_eq!(config.http.bind, "0.0.0.0");
        assert_eq!(config.clawos.log_level, "info");
        assert!(config.chain.dry_run);
        assert_eq!(config.watchdog.check_interval_secs, 10);
    }

    #[test]
    fn test_parse_minimal_toml() {
        let toml_str = r#"
[http]
port = 8080
"#;
        let config: ClawdConfig = toml::from_str(toml_str).expect("parse");
        assert_eq!(config.http.port, 8080);
        // Defaults for everything else
        assert_eq!(config.clawos.log_level, "info");
        assert!(config.chain.dry_run);
    }

    #[test]
    fn test_parse_full_toml() {
        let toml_str = r#"
[clawos]
log_level = "debug"
data_dir = "/var/lib/clawos"

[http]
bind = "127.0.0.1"
port = 9090

[mqtt]
port = 1884
max_connections = 50

[chain]
dry_run = false
rpc_url = "ws://mainnet.clawchain.io:9944"
key_path = "/etc/clawos/agent.key"

[watchdog]
check_interval_secs = 5
heartbeat_timeout_secs = 15
auto_restart = false
max_restarts = 3
"#;
        let config: ClawdConfig = toml::from_str(toml_str).expect("parse");
        assert_eq!(config.clawos.log_level, "debug");
        assert_eq!(config.http.bind, "127.0.0.1");
        assert_eq!(config.http.port, 9090);
        assert_eq!(config.mqtt.port, 1884);
        assert!(!config.chain.dry_run);
        assert!(!config.watchdog.auto_restart);
    }

    #[test]
    fn test_http_addr() {
        let config = ClawdConfig::default();
        assert_eq!(config.http_addr(), "0.0.0.0:7070");
    }

    #[test]
    fn test_load_or_default_missing_file() {
        let config = ClawdConfig::load_or_default(Path::new("/nonexistent/config.toml"));
        assert_eq!(config.http.port, 7070); // Should get defaults
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = ClawdConfig::default();
        let toml_str = toml::to_string_pretty(&config).expect("serialize");
        let parsed: ClawdConfig = toml::from_str(&toml_str).expect("parse");
        assert_eq!(config.http.port, parsed.http.port);
        assert_eq!(config.clawos.log_level, parsed.clawos.log_level);
    }
}
