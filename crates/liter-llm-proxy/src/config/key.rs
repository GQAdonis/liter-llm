use secrecy::SecretString;
use serde::Deserialize;

/// A single provider credential in a virtual key's credential pool.
///
/// When a virtual key's `provider_credentials` list is non-empty, the proxy
/// uses a [`crate::provider::InMemoryCredentialPool`] to rotate among these
/// credentials automatically on 429 / 5xx responses.
///
/// `api_key` is stored as a [`SecretString`] so the value is zeroed on drop
/// and redacted in `Debug` output.  Do **not** log or display this struct
/// directly — use `format!("{:?}", cred)` to verify redaction if needed.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderCredential {
    /// The provider name this credential is for (e.g. `"openai"`, `"anthropic"`).
    pub provider: String,
    /// Opaque identifier for this credential within the pool.
    pub id: String,
    /// The raw API key — stored behind `SecretString`; zeroed on drop.
    pub api_key: SecretString,
    /// Optional list of model names this credential is allowed to serve.
    /// `null` / omitted means the credential is valid for all models.
    #[serde(default)]
    pub model_allowlist: Option<Vec<String>>,
}

impl std::fmt::Debug for ProviderCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderCredential")
            .field("provider", &self.provider)
            .field("id", &self.id)
            .field("api_key", &"[REDACTED]")
            .field("model_allowlist", &self.model_allowlist)
            .finish()
    }
}

/// A virtual API key with optional model restrictions, rate/budget limits,
/// and a per-provider credential pool for automatic key rotation.
///
/// `key` is the raw bearer token clients present. `Debug` is implemented
/// manually below (mirroring [`ProviderCredential`]'s redaction above)
/// instead of derived, so the token never appears in a `{:?}`-formatted
/// trace event, panic message, or error context.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VirtualKeyConfig {
    pub key: String,
    pub description: Option<String>,
    /// Models this virtual key is allowed to access. Empty means all models.
    #[serde(default)]
    pub models: Vec<String>,
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
    pub budget_limit: Option<f64>,
    /// Explicit tenant identity for this key.
    ///
    /// ~keep Absent by default (`#[serde(default)]`) so existing configs keep
    /// ~keep loading unchanged. When set, every key that shares this value is
    /// ~keep billed, cached, and rate-limited as one tenant — the supported way to
    /// ~keep issue several keys against one budget, which is impossible when the
    /// ~keep tenant id is the key itself. When absent, `KeyContext::from_config`
    /// ~keep derives a stable, non-secret id from the key rather than using the
    /// ~keep raw token; see that function's doc comment for why, and for the
    /// ~keep upgrade note on existing deployments.
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// Per-provider credential pool.
    ///
    /// When non-empty, each provider listed here gets an
    /// [`crate::provider::InMemoryCredentialPool`] seeded with these entries.
    /// The proxy rotates among them automatically on 429 / 5xx responses.
    ///
    /// Example TOML:
    /// ```toml
    /// [[keys]]
    /// key = "vk-mykey"
    ///
    /// [[keys.provider_credentials]]
    /// provider = "openai"
    /// id = "key-1"
    /// api_key = "sk-..."
    ///
    /// [[keys.provider_credentials]]
    /// provider = "openai"
    /// id = "key-2"
    /// api_key = "sk-..."
    /// model_allowlist = ["gpt-4o"]
    /// ```
    #[serde(default)]
    pub provider_credentials: Vec<ProviderCredential>,
}

impl std::fmt::Debug for VirtualKeyConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualKeyConfig")
            .field("key", &"[REDACTED]")
            .field("description", &self.description)
            .field("models", &self.models)
            .field("rpm", &self.rpm)
            .field("tpm", &self.tpm)
            .field("budget_limit", &self.budget_limit)
            .field("tenant_id", &self.tenant_id)
            .field("provider_credentials", &self.provider_credentials)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_key_config_debug_redacts_key() {
        let key = VirtualKeyConfig {
            key: "vk-do-not-leak-me".to_string(),
            description: Some("team-a".to_string()),
            models: vec!["gpt-4o".to_string()],
            rpm: Some(60),
            tpm: None,
            budget_limit: None,
            tenant_id: None,
            provider_credentials: vec![],
        };

        let debug_output = format!("{key:?}");
        assert!(
            !debug_output.contains("vk-do-not-leak-me"),
            "key must not appear in Debug output: {debug_output}"
        );
        assert!(
            debug_output.contains("team-a"),
            "non-secret fields should still be visible: {debug_output}"
        );
    }

    #[test]
    fn provider_credential_debug_redacts_api_key() {
        let cred = ProviderCredential {
            provider: "openai".to_string(),
            id: "primary".to_string(),
            api_key: SecretString::from("sk-do-not-leak".to_string()),
            model_allowlist: None,
        };

        let debug_output = format!("{cred:?}");
        assert!(
            !debug_output.contains("sk-do-not-leak"),
            "api_key must not appear in Debug output: {debug_output}"
        );
        assert!(
            debug_output.contains("openai"),
            "non-secret fields should still be visible: {debug_output}"
        );
    }
}
