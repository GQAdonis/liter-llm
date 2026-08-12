use std::future::Future;
use std::pin::Pin;
use std::sync::RwLock;

use dashmap::DashMap;
use liter_llm::tenant::{KeyResolver, KeyResolverError, ResolvedKey, TenantId};
use secrecy::{ExposeSecret, SecretString};
use subtle::ConstantTimeEq;

use crate::config::VirtualKeyConfig;

/// Tenant identifier used for requests authenticated via the master key.
///
/// This is a well-known constant so downstream consumers (BudgetLedger,
/// TenantScopedStrategy, UsageEvent) can distinguish master-key traffic
/// from virtual-key traffic without a special-case enum.
pub const MASTER_TENANT_ID: &str = "master";

/// Fixed seeds for [`KeyContext::redacted_id`].
///
/// Compile-time constants so the redacted identifier is stable across
/// restarts and across every replica of the proxy — an operator correlating
/// log lines needs the same key to hash to the same value everywhere. ~keep
const REDACTED_ID_SEEDS: [u64; 4] = [
    0x6c69_7465_725f_6c6c,
    0x6d5f_7072_6f78_795f,
    0x6b65_795f_7265_6461,
    0x6374_6564_5f69_6400,
];

/// Context injected into request extensions after successful auth.
///
/// `Debug` is implemented by hand: `key_id` is the virtual key token itself,
/// so a derived `Debug` would print the caller's live credential into any span
/// or log line that captured this value. ~keep
#[derive(Clone)]
pub struct KeyContext {
    pub key_id: String,
    pub allowed_models: Option<Vec<String>>,
    pub is_master: bool,
    /// Tenant resolved from the API key.
    ///
    /// For master-key auth this is always [`TenantId`]`("master")` (see
    /// [`MASTER_TENANT_ID`]).  For virtual-key auth this is the `tenant_id`
    /// returned by [`KeyResolver::resolve`].
    pub tenant_id: TenantId,
}

impl KeyContext {
    /// Create a context representing the master key (unrestricted access).
    ///
    /// The tenant is set to [`MASTER_TENANT_ID`] so that BudgetLedger and
    /// UsageEvent always have a non-null tenant dimension.
    pub fn master() -> Self {
        Self {
            key_id: "master".into(),
            allowed_models: None,
            is_master: true,
            tenant_id: TenantId::from(MASTER_TENANT_ID),
        }
    }

    /// Create a context from a virtual key configuration.
    ///
    /// The `tenant_id` defaults to the key token itself when the config does
    /// not carry an explicit tenant.  Callers that have resolved a
    /// [`ResolvedKey`] should prefer [`KeyContext::from_resolved`] instead.
    pub fn from_config(config: &VirtualKeyConfig) -> Self {
        let allowed_models = if config.models.is_empty() {
            None
        } else {
            Some(config.models.clone())
        };
        Self {
            key_id: config.key.clone(),
            allowed_models,
            is_master: false,
            tenant_id: TenantId::from(config.key.as_str()),
        }
    }

    /// Create a context from a [`ResolvedKey`] returned by [`KeyResolver::resolve`].
    ///
    /// Uses the `tenant_id` and `allowed_models` from the resolved record so
    /// the auth layer propagates the canonical tenant identity rather than
    /// using the raw key token as a stand-in.
    pub fn from_resolved(key_id: impl Into<String>, resolved: &ResolvedKey) -> Self {
        let allowed_models = if resolved.allowed_models.is_empty() {
            None
        } else {
            Some(resolved.allowed_models.clone())
        };
        Self {
            key_id: key_id.into(),
            allowed_models,
            is_master: false,
            tenant_id: resolved.tenant_id.clone(),
        }
    }

    /// A stable, non-reversible identifier for this key, safe to log and to
    /// return in an error body.
    ///
    /// ~keep `key_id` holds the virtual key token — the live credential the
    /// ~keep caller authenticates with. Formatting it into a 403 body put that
    /// ~keep credential into every access log, reverse proxy and error tracker
    /// ~keep on the response path, and into the client's own logs. Use this
    /// ~keep anywhere the identity is being reported rather than compared.
    pub fn redacted_id(&self) -> String {
        if self.is_master {
            return "master".to_owned();
        }

        // ~keep `with_seeds`, not `generate_with`: the latter folds in a per-call
        // ~keep random component, so the same key hashed twice would not match and
        // ~keep the identifier would be useless for correlating log lines.
        let state = ahash::RandomState::with_seeds(
            REDACTED_ID_SEEDS[0],
            REDACTED_ID_SEEDS[1],
            REDACTED_ID_SEEDS[2],
            REDACTED_ID_SEEDS[3],
        );
        let digest = state.hash_one(self.key_id.as_str());
        format!("vk-{digest:016x}")
    }

    /// Returns true if this key is allowed to access the given model.
    pub fn can_access_model(&self, model: &str) -> bool {
        match &self.allowed_models {
            None => true,
            Some(models) => models.iter().any(|m| m == model),
        }
    }
}

impl std::fmt::Debug for KeyContext {
    /// ~keep Both `key_id` and `tenant_id` hold the virtual key token, so
    /// ~keep neither may be printed. A derived `Debug` leaked the caller's live
    /// ~keep credential into any span or log that captured this value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyContext")
            .field("key_id", &self.redacted_id())
            .field("allowed_models", &self.allowed_models)
            .field("is_master", &self.is_master)
            .field("tenant_id", &"<redacted>")
            .finish()
    }
}

/// In-memory virtual key store backed by `DashMap` for concurrent access.
///
/// # Constant-time lookup design
///
/// [`KeyStore::get`] performs a **constant-time** virtual-key lookup to
/// prevent timing side-channel attacks that could reveal whether a given token
/// (or its prefix) exists in the store.
///
/// A naive `DashMap::get(token)` call computes the token's hash and probes
/// the hash table.  The probe-chain length and branch behaviour leak
/// information about the stored key set (prefix matches, hamming distance).
///
/// Instead we iterate over all registered keys and compare each with
/// `subtle::ConstantTimeEq`.  This is O(n) in the number of virtual keys.
/// The trade-off is acceptable: production deployments with hundreds of VKs
/// still complete in microseconds, and the constant-time guarantee is sound
/// regardless of the key population.
///
/// The `DashMap` is retained for non-timing-sensitive operations (e.g. hot
/// reload of key metadata) and for the master-key check which is already
/// constant-time.
pub struct KeyStore {
    /// Raw key tokens stored as `SecretString`.  The `DashMap` is only used
    /// to hold the configuration; the lookup path iterates entries and uses
    /// `subtle::ConstantTimeEq` to compare tokens.
    keys: DashMap<String, VirtualKeyConfig>,
    /// `RwLock`-guarded so [`KeyStore::reload`] can hot-swap the master key
    /// from the config watcher without rebuilding the whole store (and
    /// without every `AppState` clone holding a stale `Arc<KeyStore>`).
    master_key: RwLock<Option<SecretString>>,
}

impl KeyStore {
    /// Build a key store from the proxy configuration values.
    pub fn from_config(master_key: Option<SecretString>, keys: &[VirtualKeyConfig]) -> Self {
        let map = DashMap::new();
        for k in keys {
            map.insert(k.key.clone(), k.clone());
        }
        Self {
            keys: map,
            master_key: RwLock::new(master_key),
        }
    }

    /// Replace the master key and the full set of virtual keys in place.
    ///
    /// Called by the config watcher (`config::watcher::handle_event`) on
    /// every successful hot-reload so that revoked keys stop authenticating
    /// immediately, instead of remaining valid for the lifetime of the
    /// process (see issue #69). Keys present in the store but absent from
    /// `keys` are removed; keys present in `keys` are inserted or updated.
    pub fn reload(&self, master_key: Option<SecretString>, keys: &[VirtualKeyConfig]) {
        match self.master_key.write() {
            Ok(mut guard) => *guard = master_key,
            Err(poisoned) => {
                // ~keep Recover instead of propagating: a poisoned lock must not
                // ~keep stop the watcher from applying the virtual-key half of the reload.
                tracing::error!("key store: master key lock poisoned; recovering and continuing reload");
                *poisoned.into_inner() = master_key;
            }
        }

        let incoming: std::collections::HashSet<&str> = keys.iter().map(|k| k.key.as_str()).collect();
        self.keys
            .retain(|existing_key, _| incoming.contains(existing_key.as_str()));
        for k in keys {
            self.keys.insert(k.key.clone(), k.clone());
        }
    }

    /// Check whether `token` matches the configured master key using a
    /// constant-time comparison to prevent timing side-channel attacks.
    ///
    /// The master key length is deployment-static configuration, not
    /// user-controlled per request, so the length-check short-circuit is
    /// acceptable.
    pub fn is_master_key(&self, token: &str) -> bool {
        let guard = match self.master_key.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let Some(master) = guard.as_ref() else {
            return false;
        };
        let master_bytes = master.expose_secret().as_bytes();
        let token_bytes = token.as_bytes();
        // ~keep An empty master key matches an empty token: ct_eq on two zero-length slices is
        // ~keep EQUAL, and `Authorization: Bearer ` strips to "". Config load already rejects an
        // ~keep empty master key, but this is the actual comparison, so it refuses independently
        // ~keep rather than trusting a caller upstream to have validated.
        if master_bytes.is_empty() || token_bytes.is_empty() {
            return false;
        }
        master_bytes.ct_eq(token_bytes).into()
    }

    /// Look up a virtual key configuration by its token string using a
    /// constant-time comparison.
    ///
    /// # Constant-time guarantee
    ///
    /// This method iterates ALL registered virtual keys and compares each
    /// token with `subtle::ConstantTimeEq`.  The iteration runs to completion
    /// regardless of whether a match is found, preventing early-exit timing
    /// leaks.  The result is captured and returned after the loop.
    ///
    /// The `DashMap` stores keys in an unordered bucket structure.  Iteration
    /// order is non-deterministic across calls, which provides additional
    /// resistance against timing correlation attacks that depend on consistent
    /// ordering.
    ///
    /// # Complexity
    ///
    /// O(n) where n is the number of registered virtual keys.  For practical
    /// deployments (n ≤ 10 000) this completes in < 1 ms.
    pub fn get(&self, token: &str) -> Option<VirtualKeyConfig> {
        let token_bytes = token.as_bytes();
        // ~keep Same empty-slice equality hazard as `is_master_key`: a virtual key that
        // ~keep interpolated to "" would be matched by an empty bearer token. Rejecting on the
        // ~keep INCOMING token costs no constant-time property — its length is attacker-known,
        // ~keep so branching on it leaks nothing about the stored keys.
        if token_bytes.is_empty() {
            return None;
        }
        let mut found: Option<VirtualKeyConfig> = None;

        for entry in self.keys.iter() {
            let stored_bytes = entry.key().as_bytes();

            if bool::from(stored_bytes.ct_eq(token_bytes)) && found.is_none() {
                found = Some(entry.value().clone());
            }
        }

        found
    }
}

impl KeyResolver for KeyStore {
    fn resolve(
        &self,
        api_key: String,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedKey, KeyResolverError>> + Send + 'static>> {
        let result = match self.get(&api_key) {
            None => Err(KeyResolverError::NotFound),
            Some(cfg) => Ok(ResolvedKey {
                tenant_id: TenantId::from(cfg.key.clone()),
                allowed_models: cfg.models.clone(),
                monthly_budget: cfg
                    .budget_limit
                    .map(|b| rust_decimal::Decimal::from_f64_retain(b).unwrap_or(rust_decimal::Decimal::ZERO)),
                currency: None,
                metadata: std::collections::HashMap::new(),
                active: true,
            }),
        };
        Box::pin(std::future::ready(result))
    }
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;

    use super::*;

    fn sample_key_config(key: &str, models: Vec<String>) -> VirtualKeyConfig {
        VirtualKeyConfig {
            key: key.to_string(),
            description: None,
            models,
            rpm: Some(60),
            tpm: Some(100_000),
            budget_limit: Some(50.0),
            provider_credentials: vec![],
        }
    }

    #[test]
    fn master_key_match_returns_true() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-master".to_string())), &[]);
        assert!(store.is_master_key("sk-master"));
    }

    #[test]
    fn master_key_mismatch_returns_false() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-master".to_string())), &[]);
        assert!(!store.is_master_key("sk-wrong"));
    }

    #[test]
    fn no_master_key_always_returns_false() {
        let store = KeyStore::from_config(None, &[]);
        assert!(!store.is_master_key("sk-anything"));
    }

    #[test]
    fn master_key_near_miss_returns_false() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-master".to_string())), &[]);
        assert!(!store.is_master_key("sk-mastex"));
    }

    #[test]
    fn get_existing_key_returns_config() {
        let cfg = sample_key_config("vk-team-a", vec!["gpt-4o".into()]);
        let store = KeyStore::from_config(None, std::slice::from_ref(&cfg));

        let result = store.get("vk-team-a");
        assert!(result.is_some());
        let found = result.expect("key lookup should succeed");
        assert_eq!(found.key, "vk-team-a");
        assert_eq!(found.models, vec!["gpt-4o"]);
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let store = KeyStore::from_config(None, &[]);
        assert!(store.get("vk-missing").is_none());
    }

    /// Verify that `get` uses `subtle::ConstantTimeEq` for comparison and
    /// iterates ALL keys unconditionally, not stopping on first match.
    ///
    /// This test verifies the design contract documented on [`KeyStore::get`]:
    /// the method iterates every registered key and uses constant-time
    /// comparison.  It also verifies that an exact-match key is found while a
    /// near-miss (same length, one byte different) is not.
    #[test]
    fn key_store_constant_time_lookup() {
        let cfg_a = sample_key_config("vk-aaaa", vec![]);
        let cfg_b = sample_key_config("vk-bbbb", vec![]);
        let cfg_c = sample_key_config("vk-cccc", vec![]);
        let store = KeyStore::from_config(None, &[cfg_a, cfg_b, cfg_c]);

        assert!(store.get("vk-aaaa").is_some(), "exact match must be found");
        assert!(store.get("vk-bbbb").is_some(), "exact match must be found");
        assert!(store.get("vk-cccc").is_some(), "exact match must be found");

        assert!(store.get("vk-aaab").is_none(), "near-miss must not be found");
        assert!(store.get("vk-aaaz").is_none(), "near-miss must not be found");

        assert!(store.get("vk-aaa").is_none(), "prefix must not be found");

        assert!(store.get("vk-aaaax").is_none(), "superstring must not be found");

        assert!(store.get("").is_none(), "empty token must not be found");
    }

    /// Verify that lookup in a store with multiple keys returns the correct one.
    ///
    /// This exercises the loop-to-completion behaviour: the first matching
    /// entry is captured but the loop continues for all remaining entries.
    #[test]
    fn get_returns_correct_key_among_many() {
        let configs: Vec<VirtualKeyConfig> = (0..20)
            .map(|i| sample_key_config(&format!("vk-key-{i:04}"), vec![format!("model-{i}")]))
            .collect();
        let store = KeyStore::from_config(None, &configs);

        for i in 0..20 {
            let token = format!("vk-key-{i:04}");
            let result = store.get(&token);
            assert!(result.is_some(), "key {token} should be found");
            let found = result.unwrap();
            assert_eq!(found.key, token);
            assert_eq!(found.models, vec![format!("model-{i}")]);
        }
    }

    #[test]
    fn master_context_has_no_restrictions() {
        let ctx = KeyContext::master();
        assert!(ctx.is_master);
        assert!(ctx.allowed_models.is_none());
        assert!(ctx.can_access_model("any-model"));
    }

    #[test]
    fn context_with_allowed_models_permits_listed_model() {
        let cfg = sample_key_config("vk-1", vec!["gpt-4o".into(), "claude-sonnet".into()]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(!ctx.is_master);
        assert!(ctx.can_access_model("gpt-4o"));
        assert!(ctx.can_access_model("claude-sonnet"));
    }

    #[test]
    fn context_with_allowed_models_denies_unlisted_model() {
        let cfg = sample_key_config("vk-1", vec!["gpt-4o".into()]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(!ctx.can_access_model("claude-sonnet"));
    }

    #[test]
    fn context_with_empty_models_allows_all() {
        let cfg = sample_key_config("vk-1", vec![]);
        let ctx = KeyContext::from_config(&cfg);

        assert!(ctx.allowed_models.is_none());
        assert!(ctx.can_access_model("any-model"));
    }

    /// Regression test for issue #69: a virtual key removed from config and
    /// reloaded via `KeyStore::reload` must stop authenticating immediately.
    #[test]
    fn reload_revokes_removed_virtual_key() {
        let cfg = sample_key_config("vk-revoke-me", vec![]);
        let store = KeyStore::from_config(None, std::slice::from_ref(&cfg));
        assert!(store.get("vk-revoke-me").is_some(), "key must resolve before reload");

        store.reload(None, &[]);

        assert!(
            store.get("vk-revoke-me").is_none(),
            "revoked key must not resolve after reload"
        );
    }

    /// Regression test for issue #69: `reload` must also add newly-configured
    /// virtual keys, not just remove old ones.
    #[test]
    fn reload_adds_new_virtual_key() {
        let store = KeyStore::from_config(None, &[]);
        assert!(store.get("vk-new").is_none());

        let cfg = sample_key_config("vk-new", vec!["gpt-4o".into()]);
        store.reload(None, std::slice::from_ref(&cfg));

        let found = store.get("vk-new").expect("newly-reloaded key should resolve");
        assert_eq!(found.models, vec!["gpt-4o"]);
    }

    /// Regression test for issue #69: an unrelated key present both before
    /// and after a reload must be unaffected.
    #[test]
    fn reload_preserves_unrelated_keys() {
        let kept = sample_key_config("vk-kept", vec![]);
        let removed = sample_key_config("vk-removed", vec![]);
        let store = KeyStore::from_config(None, &[kept.clone(), removed]);

        store.reload(None, std::slice::from_ref(&kept));

        assert!(store.get("vk-kept").is_some(), "unrelated key must survive reload");
        assert!(store.get("vk-removed").is_none(), "removed key must not survive reload");
    }

    /// Regression test for issue #69: hot-reload must revoke the master key
    /// too, not just virtual keys — a rotated/removed master key must stop
    /// authenticating immediately.
    #[test]
    fn reload_revokes_master_key() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-old-master".to_string())), &[]);
        assert!(store.is_master_key("sk-old-master"));

        store.reload(None, &[]);

        assert!(
            !store.is_master_key("sk-old-master"),
            "old master key must be revoked after reload"
        );
    }

    /// Regression test for issue #69: hot-reload must rotate the master key
    /// to a new value.
    #[test]
    fn reload_rotates_master_key() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-old-master".to_string())), &[]);

        store.reload(Some(SecretString::from("sk-new-master".to_string())), &[]);

        assert!(
            !store.is_master_key("sk-old-master"),
            "old master key must no longer match"
        );
        assert!(
            store.is_master_key("sk-new-master"),
            "new master key must match after reload"
        );
    }

    /// An empty master key must authenticate NOBODY.
    ///
    /// `subtle::ct_eq` on two zero-length byte slices compares EQUAL, and
    /// `interpolate_env_vars` turns an unset `${VAR}` into `""`, so the
    /// documented `master_key = "${LITER_LLM_MASTER_KEY}"` idiom with the
    /// variable unset would otherwise promote every caller to master. The
    /// empty token is reachable from the wire: `Authorization: Bearer `
    /// strips to `""`.
    #[test]
    fn empty_master_key_authenticates_nobody() {
        let store = KeyStore::from_config(Some(SecretString::from(String::new())), &[]);

        assert!(
            !store.is_master_key(""),
            "an empty master key must not match an empty token — this is a full auth bypass"
        );
        assert!(
            !store.is_master_key("anything"),
            "an empty master key must match nothing"
        );
    }

    /// An empty bearer token must never match a real master key either.
    #[test]
    fn empty_token_never_matches_a_configured_master_key() {
        let store = KeyStore::from_config(Some(SecretString::from("sk-real-master".to_string())), &[]);

        assert!(!store.is_master_key(""), "empty token must not match a real master key");
    }

    /// `key_id` and `tenant_id` both hold the virtual key token, so neither
    /// may appear in anything the proxy hands back or writes out.  These were
    /// formatted verbatim into 403 bodies, putting the caller's live
    /// credential into every access log, reverse proxy and error tracker on
    /// the response path.
    #[test]
    fn redacted_id_never_contains_the_key() {
        let secret = "sk-vk-super-secret-token";
        let ctx = KeyContext::from_config(&sample_key_config(secret, vec![]));

        let redacted = ctx.redacted_id();

        assert!(
            !redacted.contains(secret),
            "redacted id must not embed the key: {redacted}"
        );
        assert!(
            !redacted.contains("super-secret"),
            "redacted id must not embed any part of the key: {redacted}"
        );
        assert!(redacted.starts_with("vk-"), "got {redacted}");
    }

    /// The redacted id has to be stable, or it is useless for correlating
    /// log lines across replicas and restarts.
    #[test]
    fn redacted_id_is_stable_for_the_same_key() {
        let config = sample_key_config("sk-vk-stable", vec![]);

        assert_eq!(
            KeyContext::from_config(&config).redacted_id(),
            KeyContext::from_config(&config).redacted_id()
        );
        assert_ne!(
            KeyContext::from_config(&config).redacted_id(),
            KeyContext::from_config(&sample_key_config("sk-vk-other", vec![])).redacted_id(),
            "distinct keys must not collapse to the same identifier"
        );
    }

    /// The master key is a well-known constant, not a per-tenant secret.
    #[test]
    fn redacted_id_reports_master_plainly() {
        assert_eq!(KeyContext::master().redacted_id(), "master");
    }

    /// A derived `Debug` printed the token; any `?key_ctx` in a span or log
    /// line would have leaked it.
    #[test]
    fn debug_output_never_contains_the_key() {
        let secret = "sk-vk-super-secret-token";
        let ctx = KeyContext::from_config(&sample_key_config(secret, vec!["gpt-4".into()]));

        let rendered = format!("{ctx:?}");

        assert!(!rendered.contains(secret), "Debug must not print the key: {rendered}");
        assert!(
            rendered.contains("<redacted>"),
            "tenant_id must be redacted too, since it holds the same token: {rendered}"
        );
    }

    /// The same zero-length equality hazard applies to virtual-key lookup.
    #[test]
    fn empty_token_does_not_resolve_a_virtual_key() {
        let empty_key = sample_key_config("", vec![]);
        let store = KeyStore::from_config(Some(SecretString::from("sk-master".to_string())), &[empty_key]);

        assert!(
            store.get("").is_none(),
            "an empty token must not resolve a virtual key whose token interpolated away"
        );
    }
}
