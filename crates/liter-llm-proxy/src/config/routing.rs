//! Configuration surface for `[routing]` — selects the [`RoutingStrategy`]
//! applied to every multi-deployment `[[models]]` group and, for
//! `strategy = "semantic"`, the classifier that drives it.
//!
//! [`RoutingStrategy`]: liter_llm::tower::RoutingStrategy
//!
//! Absent `[routing]`, `service_pool.rs` preserves the pre-existing default:
//! every multi-deployment group is wrapped in a round-robin [`Router`].
//!
//! [`Router`]: liter_llm::tower::Router
//!
//! The `strategy` field drives which other fields are required, via serde's
//! internally-tagged enum representation — `strategy = "weighted_random"`
//! without `weights`, or `strategy = "semantic"` without `classifier`, fails
//! TOML parsing with a field-level error instead of silently falling back to
//! round-robin. An unrecognised `strategy` (or `classifier.kind`) value fails
//! the same way, listing the valid variant names.
//!
//! Unrecognised *extra* keys — a typo'd `weight` instead of `weights` — are
//! **not** caught by serde alone: `#[serde(deny_unknown_fields)]` on an
//! internally-tagged enum is a documented serde no-op, since tag dispatch
//! buffers the table's content before re-deserializing the matched variant.
//! `config::validate_routing_keys` (private, in `config/mod.rs`) closes that
//! gap with a manual TOML-level check that both `[routing]` and
//! `[routing.classifier]` go through.

use serde::Deserialize;

/// `[routing]` — see the module docs for the validate-at-parse-time contract.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, tag = "strategy", rename_all = "snake_case")]
pub enum RoutingConfig {
    /// Cycle through deployments in order using an atomic counter.
    RoundRobin,
    /// Try deployments in order; advance to the next on a transient error.
    Fallback,
    /// Route to the deployment with the lowest observed latency EMA.
    LatencyBased,
    /// Route to the cheapest deployment for the requested model.
    CostBased,
    /// Weighted-random distribution. `weights` must have one entry per
    /// deployment in every multi-deployment group this strategy applies to
    /// (`[routing]` is global — see module docs) — a mismatched length fails
    /// at `Router` construction with a clear error rather than defaulting.
    WeightedRandom {
        /// One weight per deployment, same order as the `[[models]]` entries
        /// sharing a name.
        weights: Vec<f64>,
    },
    /// Intent-based routing via the classifier cascade described by
    /// `classifier`.
    Semantic {
        /// The classifier that resolves each request to a target model ID.
        classifier: ClassifierConfig,
    },
}

/// Configuration for the single classifier behind `strategy = "semantic"`.
///
/// The Rust API's [`CascadeClassifier`](liter_llm::tower::CascadeClassifier)
/// supports composing multiple tiers (keyword → embedding → LLM); this TOML
/// surface exposes one classifier at a time. Compose tiers programmatically
/// if the cascade is needed.
///
/// `Embedding.api_key` is a raw provider credential. `Debug` is implemented
/// manually below (mirroring [`super::key::ProviderCredential`]'s redaction)
/// instead of derived, so the key never appears in a `{:?}`-formatted trace
/// event, panic message, or error context.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, tag = "kind", rename_all = "snake_case")]
pub enum ClassifierConfig {
    /// Regex-rule based routing. Rules are evaluated in order; the first
    /// matching pattern wins. An invalid regex fails at startup when the
    /// classifier is built, not at TOML-parse time (pattern syntax isn't a
    /// serde concern).
    Keyword {
        /// `(pattern, target model)` pairs evaluated in order.
        rules: Vec<KeywordRuleConfig>,
    },
    /// Cosine-similarity routing against precomputed intent-prototype
    /// embeddings.
    ///
    /// `prototypes[].embedding` must be **precomputed offline** — call the
    /// embedding provider once per example prompt and paste the resulting
    /// vector here. The proxy does not compute prototype embeddings itself;
    /// it only embeds the live request prompt (via `embedding_model`) for
    /// comparison against these vectors at request time.
    Embedding {
        /// Model used to embed the live request prompt, e.g.
        /// `"openai/text-embedding-3-small"`.
        embedding_model: String,
        /// Credential for the embedding call. Falls back to the provider's
        /// environment variable, same as `[[models]].api_key`.
        #[serde(default)]
        api_key: Option<String>,
        /// Endpoint override for the embedding call.
        #[serde(default)]
        base_url: Option<String>,
        /// Minimum cosine similarity, in `[0, 1]`, for the classifier to
        /// commit to its nearest prototype instead of deferring.
        threshold: f64,
        /// Registered intent prototypes.
        prototypes: Vec<PrototypeConfig>,
    },
}

impl std::fmt::Debug for ClassifierConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword { rules } => f.debug_struct("Keyword").field("rules", rules).finish(),
            Self::Embedding {
                embedding_model,
                api_key,
                base_url,
                threshold,
                prototypes,
            } => f
                .debug_struct("Embedding")
                .field("embedding_model", embedding_model)
                .field("api_key", &api_key.as_ref().map(|_| "[REDACTED]"))
                .field("base_url", base_url)
                .field("threshold", threshold)
                .field("prototypes", prototypes)
                .finish(),
        }
    }
}

/// A single `KeywordClassifier` rule: a regex pattern and the model it
/// routes to on match.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeywordRuleConfig {
    /// Regex pattern matched against the request's prompt text.
    pub pattern: String,
    /// Target model ID when `pattern` matches.
    pub model: String,
}

/// A single `EmbeddingSimilarityClassifier` intent prototype.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrototypeConfig {
    /// Human-readable intent name, surfaced in logs.
    pub name: String,
    /// Target model ID when this prototype is the nearest match.
    pub model: String,
    /// Precomputed embedding vector for this intent — see the `Embedding`
    /// variant's docs for how to obtain one.
    pub embedding: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProxyConfig;

    #[test]
    fn parses_round_robin() {
        let config = ProxyConfig::from_toml_str("[routing]\nstrategy = \"round_robin\"\n").expect("valid TOML");
        assert!(matches!(config.routing, Some(RoutingConfig::RoundRobin)));
    }

    #[test]
    fn parses_fallback() {
        let config = ProxyConfig::from_toml_str("[routing]\nstrategy = \"fallback\"\n").expect("valid TOML");
        assert!(matches!(config.routing, Some(RoutingConfig::Fallback)));
    }

    #[test]
    fn parses_latency_based() {
        let config = ProxyConfig::from_toml_str("[routing]\nstrategy = \"latency_based\"\n").expect("valid TOML");
        assert!(matches!(config.routing, Some(RoutingConfig::LatencyBased)));
    }

    #[test]
    fn parses_cost_based() {
        let config = ProxyConfig::from_toml_str("[routing]\nstrategy = \"cost_based\"\n").expect("valid TOML");
        assert!(matches!(config.routing, Some(RoutingConfig::CostBased)));
    }

    #[test]
    fn parses_weighted_random_with_weights() {
        let toml = r#"
[routing]
strategy = "weighted_random"
weights = [3.0, 2.0, 1.0]
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("valid TOML");
        match config.routing {
            Some(RoutingConfig::WeightedRandom { weights }) => {
                assert_eq!(weights, vec![3.0, 2.0, 1.0]);
            }
            other => panic!("expected WeightedRandom, got {other:?}"),
        }
    }

    #[test]
    fn weighted_random_without_weights_is_rejected() {
        let toml = r#"
[routing]
strategy = "weighted_random"
"#;
        let result = ProxyConfig::from_toml_str(toml);
        assert!(
            result.is_err(),
            "missing `weights` must fail to parse, not default silently"
        );
    }

    #[test]
    fn parses_semantic_with_keyword_classifier() {
        let toml = r#"
[routing]
strategy = "semantic"

[routing.classifier]
kind = "keyword"

[[routing.classifier.rules]]
pattern = "(?i)sql|database"
model = "gpt-4o"

[[routing.classifier.rules]]
pattern = "(?i)poem|haiku"
model = "claude-3-5-sonnet"
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("valid TOML");
        match config.routing {
            Some(RoutingConfig::Semantic {
                classifier: ClassifierConfig::Keyword { rules },
            }) => {
                assert_eq!(rules.len(), 2);
                assert_eq!(rules[0].pattern, "(?i)sql|database");
                assert_eq!(rules[0].model, "gpt-4o");
                assert_eq!(rules[1].model, "claude-3-5-sonnet");
            }
            other => panic!("expected Semantic/Keyword, got {other:?}"),
        }
    }

    #[test]
    fn parses_semantic_with_embedding_classifier() {
        let toml = r#"
[routing]
strategy = "semantic"

[routing.classifier]
kind = "embedding"
embedding_model = "openai/text-embedding-3-small"
threshold = 0.75

[[routing.classifier.prototypes]]
name = "coding"
model = "gpt-4o"
embedding = [1.0, 0.0, 0.0]
"#;
        let config = ProxyConfig::from_toml_str(toml).expect("valid TOML");
        match config.routing {
            Some(RoutingConfig::Semantic {
                classifier:
                    ClassifierConfig::Embedding {
                        embedding_model,
                        threshold,
                        prototypes,
                        ..
                    },
            }) => {
                assert_eq!(embedding_model, "openai/text-embedding-3-small");
                assert!((threshold - 0.75).abs() < f64::EPSILON);
                assert_eq!(prototypes.len(), 1);
                assert_eq!(prototypes[0].name, "coding");
                assert_eq!(prototypes[0].embedding, vec![1.0, 0.0, 0.0]);
            }
            other => panic!("expected Semantic/Embedding, got {other:?}"),
        }
    }

    #[test]
    fn semantic_without_classifier_is_rejected() {
        let toml = r#"
[routing]
strategy = "semantic"
"#;
        let result = ProxyConfig::from_toml_str(toml);
        assert!(
            result.is_err(),
            "missing `classifier` must fail to parse, not default silently"
        );
    }

    #[test]
    fn unknown_strategy_name_is_rejected_with_actionable_error() {
        let toml = r#"
[routing]
strategy = "bogus_strategy"
"#;
        // ~keep Matched rather than `expect_err`: that would require `ProxyConfig: Debug`, and
        // ~keep ProxyConfig deliberately has no Debug impl because it holds virtual keys and
        // ~keep provider credentials. Deriving one to satisfy a test would leak them.
        let Err(err) = ProxyConfig::from_toml_str(toml) else {
            panic!("unknown strategy name must be rejected, not silently defaulted");
        };
        assert!(
            err.contains("bogus_strategy") || err.contains("unknown variant"),
            "error should name the bad value or explain the expected set: {err}"
        );
    }

    #[test]
    fn absent_routing_table_defaults_to_none() {
        let config = ProxyConfig::from_toml_str("").expect("valid TOML");
        assert!(config.routing.is_none());
    }

    #[test]
    fn rejects_unknown_routing_field() {
        let toml = r#"
[routing]
strategy = "round_robin"
bogus_field = true
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    /// `ClassifierConfig` is also an internally-tagged enum (`tag = "kind"`),
    /// so it needs the same manual unknown-key check `RoutingConfig` does —
    /// see `super::super::validate_routing_keys` in `config/mod.rs`. All
    /// required fields are present and valid here so the failure can only
    /// come from the unknown-key check, not from a missing-field error.
    #[test]
    fn rejects_unknown_keyword_classifier_field() {
        let toml = r#"
[routing]
strategy = "semantic"

[routing.classifier]
kind = "keyword"
rules = []
bogus_field = true
"#;
        let result = ProxyConfig::from_toml_str(toml);
        assert!(
            result.is_err(),
            "an unrecognised key alongside a complete, valid `rules` field must not be silently dropped"
        );
    }

    #[test]
    fn rejects_unknown_embedding_classifier_field() {
        let toml = r#"
[routing]
strategy = "semantic"

[routing.classifier]
kind = "embedding"
embedding_model = "openai/text-embedding-3-small"
threshold = 0.75
prototypes = []
bogus_field = true
"#;
        let result = ProxyConfig::from_toml_str(toml);
        assert!(
            result.is_err(),
            "an unrecognised key alongside otherwise complete, valid fields must not be silently dropped"
        );
    }

    /// Regression test mirroring the credential-redaction fix in `key.rs` and
    /// `model.rs`: `Embedding.api_key` must never appear in `Debug` output.
    #[test]
    fn embedding_classifier_debug_redacts_api_key() {
        let cfg = ClassifierConfig::Embedding {
            embedding_model: "openai/text-embedding-3-small".to_string(),
            api_key: Some("sk-embed-do-not-leak".to_string()),
            base_url: None,
            threshold: 0.75,
            prototypes: vec![],
        };

        let debug_output = format!("{cfg:?}");
        assert!(
            !debug_output.contains("sk-embed-do-not-leak"),
            "api_key must not appear in Debug output: {debug_output}"
        );
        assert!(
            debug_output.contains("openai/text-embedding-3-small"),
            "non-secret fields should still be visible: {debug_output}"
        );
    }
}
