use serde::Deserialize;

/// MCP transport configuration.
///
/// Controls authentication behaviour for the `mcp` CLI sub-command.
///
/// For HTTP transports (`--transport http`) auth is always enforced through
/// the `validate_api_key` axum middleware — this section is ignored.
///
/// For the stdio transport a KeyContext must be established at startup time
/// because there is no per-request auth header:
///
/// * `stdio_key_id` — restrict stdio to an existing virtual key.
/// * `stdio_trust_local` — trust the local process launched by the harness
///   by default; set it to false to require an explicit virtual key. ~keep
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpConfig {
    /// Trust the stdio transport as if it were the master key.
    ///
    /// Defaults to true for the harness's local subprocess. HTTP authentication
    /// remains enforced separately. ~keep
    #[serde(default = "default_stdio_trust_local")]
    pub stdio_trust_local: bool,

    /// Use a specific virtual key as the default context for the stdio
    /// transport.  The key must exist in `[[keys]]`.
    pub stdio_key_id: Option<String>,
}

fn default_stdio_trust_local() -> bool {
    true
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            stdio_trust_local: default_stdio_trust_local(),
            stdio_key_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::McpConfig;
    use crate::config::ProxyConfig;

    #[test]
    fn should_trust_local_stdio_when_configuration_is_omitted() {
        assert!(McpConfig::default().stdio_trust_local);
        assert!(ProxyConfig::from_toml_str("").unwrap().mcp.stdio_trust_local);
        assert!(ProxyConfig::from_toml_str("[mcp]\n").unwrap().mcp.stdio_trust_local);
    }

    #[test]
    fn should_preserve_explicit_stdio_trust_opt_out_and_key() {
        let config =
            ProxyConfig::from_toml_str("[mcp]\nstdio_trust_local = false\nstdio_key_id = 'restricted'\n").unwrap();
        assert!(!config.mcp.stdio_trust_local);
        assert_eq!(config.mcp.stdio_key_id.as_deref(), Some("restricted"));
    }
}
