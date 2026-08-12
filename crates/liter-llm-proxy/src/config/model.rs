use serde::Deserialize;

/// A named model entry with optional provider overrides and fallback chain.
///
/// `api_key` is a raw provider credential. `Debug` is implemented manually
/// below (mirroring [`super::key::ProviderCredential`]'s redaction) instead
/// of derived, so the key never appears in a `{:?}`-formatted trace event,
/// panic message, or error context.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelEntry {
    pub name: String,
    pub provider_model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub fallbacks: Vec<String>,
}

impl std::fmt::Debug for ModelEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelEntry")
            .field("name", &self.name)
            .field("provider_model", &self.provider_model)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("base_url", &self.base_url)
            .field("timeout_secs", &self.timeout_secs)
            .field("fallbacks", &self.fallbacks)
            .finish()
    }
}

/// A pattern-based alias that routes model names matching `pattern` with
/// optional credential overrides.
///
/// `api_key` is a raw provider credential; see [`ModelEntry`]'s manual
/// `Debug` impl for why this one is manual too.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AliasEntry {
    pub pattern: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

impl std::fmt::Debug for AliasEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AliasEntry")
            .field("pattern", &self.pattern)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_entry_debug_redacts_api_key() {
        let entry = ModelEntry {
            name: "gpt".to_string(),
            provider_model: "openai/gpt-4o".to_string(),
            api_key: Some("sk-live-do-not-leak-me".to_string()),
            base_url: None,
            timeout_secs: None,
            fallbacks: vec![],
        };

        let debug_output = format!("{entry:?}");
        assert!(
            !debug_output.contains("sk-live-do-not-leak-me"),
            "api_key must not appear in Debug output: {debug_output}"
        );
        assert!(
            debug_output.contains("openai/gpt-4o"),
            "non-secret fields should still be visible: {debug_output}"
        );
    }

    #[test]
    fn alias_entry_debug_redacts_api_key() {
        let alias = AliasEntry {
            pattern: "anthropic/*".to_string(),
            api_key: Some("sk-anthropic-do-not-leak".to_string()),
            base_url: Some("https://api.anthropic.com".to_string()),
        };

        let debug_output = format!("{alias:?}");
        assert!(
            !debug_output.contains("sk-anthropic-do-not-leak"),
            "api_key must not appear in Debug output: {debug_output}"
        );
        assert!(
            debug_output.contains("anthropic/*"),
            "non-secret fields should still be visible: {debug_output}"
        );
    }
}
