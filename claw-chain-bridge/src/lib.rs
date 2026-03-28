// claw-chain-bridge: ClawChain JSON-RPC bridge for ClawOS
// Phase 3 — DID registration, health receipts, task polling.
// Currently a stub; will be implemented in Phase 3.

/// Placeholder for chain bridge configuration.
#[derive(Debug, Clone)]
pub struct ChainBridgeConfig {
    /// Whether to actually submit transactions (false = dry-run, log only).
    pub dry_run: bool,
    /// ClawChain RPC endpoint.
    pub rpc_url: String,
    /// Path to the Ed25519 keypair.
    pub key_path: String,
}

impl Default for ChainBridgeConfig {
    fn default() -> Self {
        Self {
            dry_run: true,
            rpc_url: "ws://127.0.0.1:9944".to_string(),
            key_path: "~/.clawd/keys/agent.key".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_dry_run() {
        let config = ChainBridgeConfig::default();
        assert!(config.dry_run);
    }
}
