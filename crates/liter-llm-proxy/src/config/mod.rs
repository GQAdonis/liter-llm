pub mod files;
pub mod key;
pub mod mcp;
pub mod model;
pub mod provider;
pub mod routing;
pub mod security;
pub mod server;
pub mod watcher;

pub use files::FileStorageConfig;
pub use key::VirtualKeyConfig;
pub use mcp::McpConfig;
pub use model::{AliasEntry, ModelEntry};
#[cfg(feature = "etcd-watch")]
pub use provider::EtcdConfigProvider;
pub use provider::{ConfigError, ConfigEvent, ConfigProvider, FileWatchConfigProvider, StaticFileConfigProvider};
pub use routing::{ClassifierConfig, KeywordRuleConfig, PrototypeConfig, RoutingConfig};
pub use security::{OutboundPolicyKind, SecurityConfig};
pub use server::ServerConfig;

use std::collections::HashMap;
use std::path::Path;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::auth::MASTER_TENANT_ID;

fn default_timeout() -> u64 {
    120
}

fn default_retries() -> u32 {
    3
}

fn default_cache_backend() -> String {
    "memory".to_string()
}

/// General proxy behaviour: master key, timeouts, retries, feature flags.
///
/// Note: `master_key` is a [`SecretString`]; its `Debug` output is redacted.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    pub master_key: Option<SecretString>,
    #[serde(default = "default_timeout")]
    pub default_timeout_secs: u64,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    #[serde(default)]
    pub enable_cost_tracking: bool,
    #[serde(default)]
    pub enable_tracing: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            master_key: None,
            default_timeout_secs: default_timeout(),
            max_retries: default_retries(),
            enable_cost_tracking: false,
            enable_tracing: false,
        }
    }
}

/// Global rate-limit settings (requests-per-minute / tokens-per-minute).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateLimitConfig {
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
}

/// How budget limits are enforced.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EnforcementMode {
    /// Requests exceeding the budget are rejected.
    #[default]
    Hard,
    /// Requests exceeding the budget are logged but allowed through.
    Soft,
}

/// Budget enforcement settings with optional per-model limits.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetConfig {
    pub global_limit: Option<f64>,
    #[serde(default)]
    pub model_limits: HashMap<String, f64>,
    #[serde(default)]
    pub enforcement: EnforcementMode,
}

/// Semantic cache configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    pub max_entries: Option<usize>,
    pub ttl_seconds: Option<u64>,
    #[serde(default = "default_cache_backend")]
    pub backend: String,
    #[serde(default)]
    pub backend_config: HashMap<String, String>,
}

/// Periodic health-check probe settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthConfig {
    pub interval_secs: Option<u64>,
    pub probe_model: Option<String>,
}

/// Provider cooldown duration after consecutive failures.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CooldownConfig {
    pub duration_secs: u64,
}

/// Root configuration for the liter-llm proxy server.
///
/// Loaded from a `liter-llm-proxy.toml` file. After deserialization all
/// `${VAR_NAME}` patterns in string values are replaced with the
/// corresponding environment variable.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub models: Vec<ModelEntry>,
    #[serde(default)]
    pub aliases: Vec<AliasEntry>,
    pub rate_limit: Option<RateLimitConfig>,
    pub budget: Option<BudgetConfig>,
    pub cache: Option<CacheConfig>,
    pub files: Option<FileStorageConfig>,
    #[serde(default)]
    pub keys: Vec<VirtualKeyConfig>,
    pub health: Option<HealthConfig>,
    pub cooldown: Option<CooldownConfig>,
    #[serde(default)]
    pub mcp: McpConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    /// Routing strategy applied to every multi-deployment `[[models]]`
    /// group. Absent means round-robin, matching pre-existing behaviour.
    /// See [`routing::RoutingConfig`].
    pub routing: Option<RoutingConfig>,
}

/// Replace all `${VAR_NAME}` occurrences in `s` with the value of the
/// corresponding environment variable. Unknown variables are replaced with
/// the empty string.
pub fn interpolate_env_vars(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'{') {
            chars.next();
            let mut var_name = String::new();
            let mut found_closing = false;
            for c in chars.by_ref() {
                if c == '}' {
                    found_closing = true;
                    break;
                }
                var_name.push(c);
            }
            if found_closing {
                if let Ok(val) = std::env::var(&var_name) {
                    result.push_str(&val);
                }
            } else {
                result.push('$');
                result.push('{');
                result.push_str(&var_name);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Keys accepted in `[routing]` for each `strategy` value.
///
/// `strategy` itself is always permitted; the second element lists the extra
/// keys that strategy takes.
const ROUTING_KEYS_BY_STRATEGY: &[(&str, &[&str])] = &[
    ("round_robin", &[]),
    ("fallback", &[]),
    ("latency_based", &[]),
    ("cost_based", &[]),
    ("weighted_random", &["weights"]),
    ("semantic", &["classifier"]),
];

/// Keys accepted in `[routing.classifier]` for each `kind` value.
///
/// `kind` itself is always permitted; the second element lists the extra
/// keys that kind takes.
const CLASSIFIER_KEYS_BY_KIND: &[(&str, &[&str])] = &[
    ("keyword", &["rules"]),
    (
        "embedding",
        &["embedding_model", "api_key", "base_url", "threshold", "prototypes"],
    ),
];

/// Reject unknown keys in the `[routing]` table and, when
/// `strategy = "semantic"`, in the nested `[routing.classifier]` table.
///
/// `RoutingConfig` and `ClassifierConfig` both carry
/// `#[serde(deny_unknown_fields)]`, but serde ignores it on an
/// internally-tagged enum (`tag = "strategy"` / `tag = "kind"`) because tag
/// dispatch buffers the content first. Without this check a misspelled key —
/// `weight` instead of `weights`, or `threshold` with a letter dropped —
/// parses successfully and is silently discarded, which is exactly the
/// silent-misconfiguration failure this config surface exists to prevent. ~keep
fn validate_routing_keys(expanded: &str) -> Result<(), String> {
    // ~keep The typed parse has already succeeded by the time this runs, so a failure here
    // ~keep would mean the two parsers disagree — surface it rather than skipping the check.
    let root: toml::Table =
        toml::from_str(expanded).map_err(|e| format!("invalid TOML config: re-parse for validation failed: {e}"))?;
    let Some(routing) = root.get("routing").and_then(toml::Value::as_table) else {
        return Ok(());
    };
    reject_unknown_keys(routing, "strategy", ROUTING_KEYS_BY_STRATEGY, "[routing]")?;

    if let Some(classifier) = routing.get("classifier").and_then(toml::Value::as_table) {
        reject_unknown_keys(classifier, "kind", CLASSIFIER_KEYS_BY_KIND, "[routing.classifier]")?;
    }
    Ok(())
}

/// Reject keys in `table` other than `tag_field` and the extra keys
/// registered for `table[tag_field]`'s value in `allowed`.
///
/// A no-op — rather than an error — when `tag_field` is absent, not a
/// string, or its value isn't a recognised tag: those cases either mean
/// there is nothing to validate here, or they already failed serde's own
/// typed parse in [`parse_with_env_interpolation`] before this runs.
fn reject_unknown_keys(
    table: &toml::Table,
    tag_field: &str,
    allowed: &[(&str, &[&str])],
    context: &str,
) -> Result<(), String> {
    let Some(tag_value) = table.get(tag_field).and_then(toml::Value::as_str) else {
        return Ok(());
    };
    let Some((_, extra)) = allowed.iter().find(|(name, _)| *name == tag_value) else {
        return Ok(());
    };

    let unknown: Vec<&str> = table
        .keys()
        .map(String::as_str)
        .filter(|key| *key != tag_field && !extra.contains(key))
        .collect();

    if unknown.is_empty() {
        return Ok(());
    }
    let mut allowed_keys = vec![tag_field];
    allowed_keys.extend_from_slice(extra);
    Err(format!(
        "invalid TOML config: unknown key(s) in {context} for {tag_field} \"{tag_value}\": {}. Accepted keys: {}",
        unknown.join(", "),
        allowed_keys.join(", ")
    ))
}

/// Apply env-var interpolation to a raw TOML string, then deserialize.
///
/// This is the simplest correct approach: interpolate the whole TOML source
/// before parsing, so every string value (including nested tables and arrays)
/// gets expanded uniformly.
fn parse_with_env_interpolation(raw: &str) -> Result<ProxyConfig, String> {
    let expanded = interpolate_env_vars(raw);
    let config: ProxyConfig = toml::from_str(&expanded).map_err(|e| format!("invalid TOML config: {e}"))?;
    validate_routing_keys(&expanded)?;
    validate_secrets_non_empty(&config)?;
    validate_virtual_key_tenant_ids(&config)?;
    Ok(config)
}

/// Reject credentials that interpolated away to the empty string.
///
/// `interpolate_env_vars` substitutes an unset `${VAR}` with `""`, and the
/// documented idiom for every secret in this file is `key = "${SOME_VAR}"`.
/// An empty master key is not a weak key, it is a total authentication bypass:
/// `KeyStore::is_master_key` compares with `ct_eq`, two zero-length byte slices
/// compare EQUAL, and `Authorization: Bearer ` (trailing space) strips to `""`
/// — so any unauthenticated caller is promoted to master. Fail at load rather
/// than serve in that state. ~keep
fn validate_secrets_non_empty(config: &ProxyConfig) -> Result<(), String> {
    if config
        .general
        .master_key
        .as_ref()
        .is_some_and(|k| k.expose_secret().is_empty())
    {
        return Err(
            "[general] master_key is set but empty — an unset ${VAR} interpolates to \"\", and an empty master key \
             authenticates every request. Set the variable or remove the key."
                .to_string(),
        );
    }
    for key in &config.keys {
        if key.key.is_empty() {
            return Err(format!(
                "virtual key '{}' has an empty token — an unset ${{VAR}} interpolates to \"\". Set the variable or \
                 remove the key.",
                key.description.as_deref().unwrap_or("<no description>")
            ));
        }
    }
    Ok(())
}

/// Reject a virtual key's explicit `tenant_id` when it is empty or collides
/// with [`MASTER_TENANT_ID`].
///
/// An empty value is the same interpolated-away-`${VAR}` hazard as
/// `validate_secrets_non_empty` guards against for `key`. `"master"` is
/// reserved: `KeyContext::master()` always resolves to `MASTER_TENANT_ID`, so
/// a virtual key configured to share it would fold that key's spend into the
/// master key's budget-ledger bucket and cache namespace — a budget-tracking
/// bypass introduced by config, not by code. ~keep
fn validate_virtual_key_tenant_ids(config: &ProxyConfig) -> Result<(), String> {
    for key in &config.keys {
        let Some(tenant_id) = key.tenant_id.as_deref() else {
            continue;
        };
        let name = key.description.as_deref().unwrap_or("<no description>");
        if tenant_id.is_empty() {
            return Err(format!(
                "virtual key '{name}' has an empty tenant_id — an unset ${{VAR}} interpolates to \"\". Set the \
                 variable or remove the tenant_id key."
            ));
        }
        if tenant_id == MASTER_TENANT_ID {
            return Err(format!(
                "virtual key '{name}' sets tenant_id = \"{MASTER_TENANT_ID}\", which is reserved for master-key \
                 traffic. Choose a different tenant_id."
            ));
        }
    }
    Ok(())
}

impl ProxyConfig {
    /// Parse from a TOML string with env-var interpolation.
    pub fn from_toml_str(s: &str) -> Result<Self, String> {
        parse_with_env_interpolation(s)
    }

    /// Load from a TOML file path with env-var interpolation.
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("failed to read config file {}: {e}", path.display()))?;
        Self::from_toml_str(&content)
    }

    /// Discover `liter-llm-proxy.toml` by walking from the current directory
    /// up to the filesystem root.
    ///
    /// Returns `Ok(None)` if no config file is found.
    pub fn discover() -> Result<Option<Self>, String> {
        let mut current = std::env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;
        loop {
            let config_path = current.join("liter-llm-proxy.toml");
            if config_path.exists() {
                return Ok(Some(Self::from_toml_file(config_path)?));
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::*;

    #[test]
    fn parse_minimal_config() {
        let config = ProxyConfig::from_toml_str("").expect("TOML config should parse");
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 4000);
        assert_eq!(config.general.default_timeout_secs, 120);
        assert_eq!(config.general.max_retries, 3);
        assert!(config.models.is_empty());
        assert!(config.keys.is_empty());
        assert!(config.rate_limit.is_none());
        assert!(config.budget.is_none());
        assert!(config.cache.is_none());
        assert!(config.files.is_none());
        assert!(config.health.is_none());
        assert!(config.cooldown.is_none());
    }

    #[test]
    fn parse_full_config() {
        let toml = r#"
[server]
host = "127.0.0.1"
port = 8080
request_timeout_secs = 300
body_limit_bytes = 5242880
cors_origins = ["https://example.com"]

[general]
master_key = "sk-master"
default_timeout_secs = 60
max_retries = 5
enable_cost_tracking = true
enable_tracing = true

[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "sk-openai"
base_url = "https://api.openai.com/v1"
timeout_secs = 30
fallbacks = ["claude-sonnet"]

[[models]]
name = "claude-sonnet"
provider_model = "anthropic/claude-sonnet-4-20250514"

[[aliases]]
pattern = "anthropic/*"
api_key = "sk-anthropic"

[[keys]]
key = "vk-team-a"
description = "Team A key"
models = ["gpt-4o"]
rpm = 60
tpm = 100000
budget_limit = 50.0

[rate_limit]
rpm = 120
tpm = 500000

[budget]
global_limit = 100.0
enforcement = "soft"

[budget.model_limits]
"openai/gpt-4o" = 50.0

[cache]
max_entries = 1024
ttl_seconds = 600
backend = "memory"

[files]
backend = "s3"
prefix = "proxy-files/"

[files.backend_config]
bucket = "my-bucket"

[health]
interval_secs = 30
probe_model = "openai/gpt-4o-mini"

[cooldown]
duration_secs = 60
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");

        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.request_timeout_secs, 300);
        assert_eq!(config.server.body_limit_bytes, 5_242_880);
        assert_eq!(config.server.cors_origins, vec!["https://example.com"]);

        assert_eq!(
            config.general.master_key.as_ref().map(|s| s.expose_secret()),
            Some("sk-master")
        );
        assert_eq!(config.general.default_timeout_secs, 60);
        assert_eq!(config.general.max_retries, 5);
        assert!(config.general.enable_cost_tracking);
        assert!(config.general.enable_tracing);

        assert_eq!(config.models.len(), 2);
        assert_eq!(config.models[0].name, "gpt-4o");
        assert_eq!(config.models[0].provider_model, "openai/gpt-4o");
        assert_eq!(config.models[0].api_key.as_deref(), Some("sk-openai"));
        assert_eq!(config.models[0].fallbacks, vec!["claude-sonnet"]);
        assert_eq!(config.models[1].name, "claude-sonnet");
        assert!(config.models[1].api_key.is_none());

        assert_eq!(config.aliases.len(), 1);
        assert_eq!(config.aliases[0].pattern, "anthropic/*");

        assert_eq!(config.keys.len(), 1);
        assert_eq!(config.keys[0].key, "vk-team-a");
        assert_eq!(config.keys[0].models, vec!["gpt-4o"]);
        assert_eq!(config.keys[0].rpm, Some(60));
        assert_eq!(
            config.keys[0].tenant_id, None,
            "a config with no tenant_id key must keep working and default to None"
        );

        let rl = config.rate_limit.expect("rate_limit should be present");
        assert_eq!(rl.rpm, Some(120));
        assert_eq!(rl.tpm, Some(500_000));

        let budget = config.budget.expect("budget should be present");
        assert_eq!(budget.global_limit, Some(100.0));
        assert_eq!(budget.enforcement, EnforcementMode::Soft);
        assert_eq!(budget.model_limits.get("openai/gpt-4o"), Some(&50.0));

        let cache = config.cache.expect("cache should be present");
        assert_eq!(cache.max_entries, Some(1024));
        assert_eq!(cache.ttl_seconds, Some(600));
        assert_eq!(cache.backend, "memory");

        let files = config.files.expect("files should be present");
        assert_eq!(files.backend, "s3");
        assert_eq!(files.prefix, "proxy-files/");
        assert_eq!(
            files.backend_config.get("bucket").expect("bucket should be present"),
            "my-bucket"
        );

        let health = config.health.expect("health should be present");
        assert_eq!(health.interval_secs, Some(30));
        assert_eq!(health.probe_model.as_deref(), Some("openai/gpt-4o-mini"));

        assert_eq!(config.cooldown.expect("cooldown should be present").duration_secs, 60);
    }

    /// A `master_key` whose `${VAR}` is unset interpolates to `""`, and an
    /// empty master key authenticates every caller. Refuse to load rather than
    /// start a proxy that is open to the world.
    #[test]
    fn rejects_master_key_that_interpolated_to_empty() {
        let toml = r#"
[general]
master_key = "${SURELY_NONEXISTENT_MASTER_KEY_VAR_98765}"
"#;
        let Err(err) = ProxyConfig::from_toml_str(toml) else {
            panic!("an empty master key must be rejected — it authenticates every request");
        };
        assert!(
            err.contains("master_key") && err.contains("empty"),
            "error must name the offending key and why: {err}"
        );
    }

    /// Same hazard one level down: a virtual key token that interpolated away.
    #[test]
    fn rejects_virtual_key_that_interpolated_to_empty() {
        let toml = r#"
[general]
master_key = "sk-real-master"

[[keys]]
key = "${SURELY_NONEXISTENT_VIRTUAL_KEY_VAR_98765}"
description = "billing-team"
"#;
        let Err(err) = ProxyConfig::from_toml_str(toml) else {
            panic!("an empty virtual key token must be rejected");
        };
        assert!(
            err.contains("billing-team"),
            "error must identify WHICH key is empty so an operator can fix it: {err}"
        );
    }

    /// An operator-set `tenant_id` must parse and be distinguishable from the
    /// key token — the config-level way to name a tenant directly instead of
    /// letting the key double as its own tenant id.
    #[test]
    fn parses_explicit_virtual_key_tenant_id() {
        let toml = r#"
[[keys]]
key = "vk-team-a-user-1"
tenant_id = "team-a"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.keys[0].tenant_id.as_deref(), Some("team-a"));
    }

    /// Same interpolated-away-`${VAR}` hazard as the key token itself: an
    /// empty tenant_id must be rejected at load, not silently accepted.
    #[test]
    fn rejects_virtual_key_tenant_id_that_interpolated_to_empty() {
        let toml = r#"
[[keys]]
key = "vk-team-a"
description = "billing-team"
tenant_id = "${SURELY_NONEXISTENT_TENANT_ID_VAR_98765}"
"#;
        let Err(err) = ProxyConfig::from_toml_str(toml) else {
            panic!("an empty tenant_id must be rejected");
        };
        assert!(
            err.contains("billing-team") && err.contains("tenant_id"),
            "error must identify WHICH key and WHY: {err}"
        );
    }

    /// `"master"` is reserved for `KeyContext::master()`. A virtual key
    /// configured to share it would fold its spend into the master tenant's
    /// budget-ledger bucket — a config-introduced budget bypass.
    #[test]
    fn rejects_virtual_key_tenant_id_that_collides_with_master() {
        let toml = r#"
[[keys]]
key = "vk-team-a"
description = "billing-team"
tenant_id = "master"
"#;
        let Err(err) = ProxyConfig::from_toml_str(toml) else {
            panic!("tenant_id = \"master\" must be rejected — it collides with the reserved master tenant");
        };
        assert!(
            err.contains("billing-team") && err.contains("master"),
            "error must identify WHICH key and WHY: {err}"
        );
    }

    /// The guard must not reject a legitimately absent master key — a proxy
    /// with only virtual keys configured is a supported deployment.
    #[test]
    fn absent_master_key_is_still_allowed() {
        let toml = r#"
[general]
default_timeout_secs = 30
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("a config with no master key must still load");
        assert!(config.general.master_key.is_none());
    }

    #[test]
    fn env_var_interpolation() {
        // ~keep SAFETY: this test does not run concurrently with users of these env vars.
        unsafe {
            std::env::set_var("LITER_TEST_KEY", "sk-from-env");
            std::env::set_var("LITER_TEST_HOST", "10.0.0.1");
        }

        let toml = r#"
[server]
host = "${LITER_TEST_HOST}"

[general]
master_key = "${LITER_TEST_KEY}"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.server.host, "10.0.0.1");
        assert_eq!(
            config.general.master_key.as_ref().map(|s| s.expose_secret()),
            Some("sk-from-env")
        );

        // ~keep SAFETY: cleaning up test-only env vars.
        unsafe {
            std::env::remove_var("LITER_TEST_KEY");
            std::env::remove_var("LITER_TEST_HOST");
        }
    }

    #[test]
    fn env_var_interpolation_preserves_literals() {
        let toml = r#"
[server]
host = "literal-value"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.server.host, "literal-value");
    }

    #[test]
    fn env_var_interpolation_unknown_var_becomes_empty() {
        let result = interpolate_env_vars("prefix-${SURELY_NONEXISTENT_VAR_12345}-suffix");
        assert_eq!(result, "prefix--suffix");
    }

    #[test]
    fn rejects_unknown_top_level_field() {
        let toml = r#"
unknown_field = true
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn rejects_unknown_server_field() {
        let toml = r#"
[server]
host = "0.0.0.0"
bogus = 42
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn rejects_unknown_general_field() {
        let toml = r#"
[general]
unknown_option = true
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn default_values_applied() {
        let config = ProxyConfig::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 4000);
        assert_eq!(config.server.request_timeout_secs, 600);
        assert_eq!(config.server.body_limit_bytes, 10_485_760);
        assert!(config.server.cors_origins.is_empty());
        assert_eq!(config.general.default_timeout_secs, 120);
        assert_eq!(config.general.max_retries, 3);
        assert!(!config.general.enable_cost_tracking);
        assert!(!config.general.enable_tracing);
    }

    #[test]
    fn budget_default_enforcement() {
        let toml = r#"
[budget]
global_limit = 100.0
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(
            config.budget.expect("budget should be present").enforcement,
            EnforcementMode::Hard
        );
    }

    #[test]
    fn cache_default_backend() {
        let toml = r#"
[cache]
max_entries = 256
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.cache.expect("cache should be present").backend, "memory");
    }

    #[test]
    fn files_default_values() {
        let toml = r#"
[files]
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        let files = config.files.expect("files should be present");
        assert_eq!(files.backend, "memory");
        assert_eq!(files.prefix, "liter-llm-files/");
        assert!(files.backend_config.is_empty());
    }

    #[test]
    fn multiple_models_same_name() {
        let toml = r#"
[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "sk-key-1"

[[models]]
name = "gpt-4o"
provider_model = "azure/gpt-4o"
api_key = "sk-key-2"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.models.len(), 2);
        assert_eq!(config.models[0].name, "gpt-4o");
        assert_eq!(config.models[1].name, "gpt-4o");
        assert_ne!(config.models[0].provider_model, config.models[1].provider_model);
    }

    #[test]
    fn model_with_fallbacks() {
        let toml = r#"
[[models]]
name = "primary"
provider_model = "openai/gpt-4o"
fallbacks = ["fallback-1", "fallback-2"]

[[models]]
name = "fallback-1"
provider_model = "anthropic/claude-sonnet-4-20250514"

[[models]]
name = "fallback-2"
provider_model = "groq/llama3-70b"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("TOML config should parse");
        assert_eq!(config.models[0].fallbacks, vec!["fallback-1", "fallback-2"]);
        assert!(config.models[1].fallbacks.is_empty());
        assert!(config.models[2].fallbacks.is_empty());
    }

    #[test]
    fn interpolate_env_vars_basic() {
        assert_eq!(interpolate_env_vars("no vars here"), "no vars here");
        assert_eq!(interpolate_env_vars(""), "");
        assert_eq!(interpolate_env_vars("$not_a_var"), "$not_a_var");
    }

    #[test]
    fn interpolate_env_vars_multiple() {
        // ~keep SAFETY: this test does not run concurrently with users of these env vars.
        unsafe {
            std::env::set_var("LITER_A", "hello");
            std::env::set_var("LITER_B", "world");
        }
        let result = interpolate_env_vars("${LITER_A} ${LITER_B}!");
        assert_eq!(result, "hello world!");
        // ~keep SAFETY: cleaning up test-only env vars.
        unsafe {
            std::env::remove_var("LITER_A");
            std::env::remove_var("LITER_B");
        }
    }

    #[test]
    fn interpolate_env_vars_unclosed_brace_treated_as_literal() {
        assert_eq!(interpolate_env_vars("prefix-${UNCLOSED"), "prefix-${UNCLOSED");
        assert_eq!(interpolate_env_vars("${"), "${");
        assert_eq!(interpolate_env_vars("a${b"), "a${b");
    }
}
