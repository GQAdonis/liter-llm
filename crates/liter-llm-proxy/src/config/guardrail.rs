//! Configuration surface for `[[guardrails]]` — the content-filtering and
//! policy checks the proxy enforces on every request.
//!
//! Each `[[guardrails]]` entry maps to one [`Guardrail`] implementation from
//! `liter_llm::guardrail`. Entries are evaluated in declaration order and the
//! first `Block` decision short-circuits, so order the cheap checks first.
//!
//! [`Guardrail`]: liter_llm::guardrail::Guardrail
//!
//! # Where the configured set is enforced
//!
//! One set is built at startup by [`crate::guardrail::build_registry`] and
//! shared by **both** request paths — the unary Tower stack in
//! [`crate::service_pool`] and the realtime WebSocket proxy in
//! [`crate::routes::realtime`]. There is deliberately no way to scope an entry
//! to one path: a guardrail an operator declared is a guardrail that runs
//! everywhere content flows.
//!
//! # Fail-closed contract
//!
//! Nothing here degrades. A `[[guardrails]]` entry that cannot be built — an
//! invalid regex, an invalid CEL expression, an unrecognised `type`, an empty
//! `stages` list, a `type = "cel"` entry in a build without the `guardrail-cel`
//! feature — aborts startup in [`crate::guardrail::build_registry`], which
//! [`crate::service_pool::ServicePool::from_config`] propagates before the
//! listener is bound. The proxy never runs with a partially-applied guardrail
//! set, because a partial set is indistinguishable from an unguarded proxy.
//!
//! # Why `allow_list` / `deny_list` are not exposed
//!
//! `liter_llm::guardrail::builtin` also ships `AllowListGuardrail` and
//! `DenyListGuardrail`, which decide on a named field of
//! `GuardrailContext::metadata`. Both paths now populate that map, but with
//! exactly ONE key: `tenant_id`. The unary path derives it in
//! `GuardrailService`; the realtime relay assembles the equivalent in
//! `routes::realtime::session_guardrail_metadata`.
//!
//! So a list guardrail keyed on `tenant_id` would work — and one keyed on any
//! other field would not, silently and asymmetrically. `deny_list` reads a
//! missing field as "nothing to deny" and never blocks; `allow_list` reads it
//! as "required field absent" and blocks everything. A config surface that
//! accepts an arbitrary `field` string would therefore hand operators a
//! control that is live or dead depending on a value they cannot validate.
//!
//! Exposing them needs the `field` value constrained to the keys actually
//! populated, rejected at load like every other malformed entry here, so an
//! unpopulated field fails startup instead of failing silently at runtime.
//! Until then these types stay reachable from Rust and absent from TOML. ~keep
//!
//! # `stages` is required, never defaulted
//!
//! For the entry types that accept it, `stages` has no serde default. Guessing
//! would mean either silently leaving the response side unguarded or silently
//! guarding more than was asked; a missing-field parse error asks the operator
//! instead.
//!
//! # Example
//!
//! ```toml
//! [[guardrails]]
//! type = "regex"
//! name = "block-ssn"
//! pattern = '\b\d{3}-\d{2}-\d{4}\b'
//! stages = ["input", "output"]
//! action = { kind = "block", code = 1100, reason_prefix = "US SSN detected" }
//!
//! [[guardrails]]
//! type = "regex"
//! name = "redact-emails"
//! pattern = '[\w.+-]+@[\w-]+\.[\w.]+'
//! stages = ["output", "output_chunk"]
//! action = { kind = "redact", replacement = "[REDACTED]" }
//!
//! [[guardrails]]
//! type = "length_cap"
//! name = "cap-prompt"
//! max_chars = 20000
//! stages = ["input"]
//!
//! [[guardrails]]
//! type = "prompt_injection"
//! name = "injection-heuristic"
//!
//! # Requires the `guardrail-cel` feature; a build without it rejects this at startup.
//! [[guardrails]]
//! type = "cel"
//! name = "no-forced-tools"
//! expression = 'has(request.tool_choice)'
//! stages = ["input"]
//! action = { kind = "block", code = 1200, reason = "forced tool use is not permitted" }
//! ```

use serde::Deserialize;

/// Pipeline stage at which a configured guardrail runs.
///
/// Mirrors [`liter_llm::guardrail::GuardrailStage`]; the conversion lives in
/// [`crate::guardrail`] so this module stays a pure config surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardrailStageConfig {
    /// The outgoing request, before it reaches the upstream provider.
    Input,
    /// The complete non-streaming response from the upstream provider.
    Output,
    /// A single chunk of a streaming response, or a single realtime event.
    OutputChunk,
}

/// What a `type = "regex"` guardrail does when its pattern matches.
///
/// Maps to [`liter_llm::guardrail::builtin::OnMatch`].
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, tag = "kind", rename_all = "snake_case")]
pub enum RegexActionConfig {
    /// Reject the request or response.
    Block {
        /// Numeric error code surfaced to the caller. Use `>= 1000` so it
        /// cannot be confused with an HTTP status.
        code: u32,
        /// Reason prefix; the matched text is appended by the guardrail.
        reason_prefix: String,
    },
    /// Replace every match with `replacement` and let the payload through.
    Redact {
        /// Text substituted in place of each match.
        replacement: String,
    },
}

/// What a `type = "cel"` guardrail does when its expression evaluates to `true`.
///
/// Maps to `liter_llm::guardrail::cel::CelAction`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, tag = "kind", rename_all = "snake_case")]
pub enum CelActionConfig {
    /// Reject the request or response.
    Block {
        /// Numeric error code surfaced to the caller. Use `>= 1000`.
        code: u32,
        /// Human-readable rejection reason.
        reason: String,
    },
    /// Replace the payload with a fixed JSON value.
    Mutate {
        /// The replacement payload.
        new_payload: serde_json::Value,
    },
}

/// One `[[guardrails]]` entry.
///
/// `type` selects the variant, following the same internally-tagged idiom as
/// [`super::routing::RoutingConfig`] — an unrecognised `type` fails TOML
/// parsing with the list of valid values rather than being skipped.
///
/// Note that `deny_unknown_fields` is a documented no-op on an
/// internally-tagged enum, so a misspelled key would otherwise parse and be
/// discarded. `config::validate_guardrail_keys` (private, in `config/mod.rs`)
/// closes that gap. ~keep
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, tag = "type", rename_all = "snake_case")]
pub enum GuardrailEntry {
    /// Block or redact content matching a regular expression, checked against
    /// the serialized request JSON, response JSON, or raw chunk text.
    Regex {
        /// Identifier used in logs and metric labels. Must be unique.
        name: String,
        /// Rust `regex` crate syntax. An invalid pattern fails startup.
        pattern: String,
        /// Stages this guardrail runs at. Must be non-empty.
        stages: Vec<GuardrailStageConfig>,
        /// What to do on a match.
        action: RegexActionConfig,
    },
    /// Evaluate a CEL expression against the guardrail context and apply
    /// `action` when it returns `true`.
    ///
    /// Requires the `guardrail-cel` feature. A build without it rejects this
    /// entry at startup rather than skipping it.
    Cel {
        /// Identifier used in logs and metric labels. Must be unique.
        name: String,
        /// CEL expression, compiled at startup. Invalid syntax fails startup.
        expression: String,
        /// Stages this guardrail runs at. Must be non-empty.
        stages: Vec<GuardrailStageConfig>,
        /// What to do when the expression evaluates to `true`.
        action: CelActionConfig,
        /// When `true`, runtime evaluation errors allow the payload through
        /// instead of blocking.
        ///
        /// Defaults to `false` (fail closed). Setting it to `true` makes any
        /// input that provokes an eval error a bypass for this guardrail; it
        /// exists for non-production environments where checks are advisory.
        #[serde(default)]
        fail_open: bool,
    },
    /// Block payloads longer than `max_chars` characters.
    LengthCap {
        /// Identifier used in logs and metric labels. Must be unique.
        name: String,
        /// Maximum inspected-text length, in characters.
        max_chars: usize,
        /// Stages this guardrail runs at. Must be non-empty.
        stages: Vec<GuardrailStageConfig>,
    },
    /// Heuristic detector for common prompt-injection phrasings.
    ///
    /// Runs at the `Input` stage only, as fixed by the core implementation.
    PromptInjection {
        /// Identifier used in logs and metric labels. Must be unique.
        name: String,
    },
}

impl GuardrailEntry {
    /// The operator-supplied `name` of this entry.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Regex { name, .. }
            | Self::Cel { name, .. }
            | Self::LengthCap { name, .. }
            | Self::PromptInjection { name } => name,
        }
    }

    /// The `type` tag of this entry, as written in TOML.
    #[must_use]
    pub fn type_label(&self) -> &'static str {
        match self {
            Self::Regex { .. } => "regex",
            Self::Cel { .. } => "cel",
            Self::LengthCap { .. } => "length_cap",
            Self::PromptInjection { .. } => "prompt_injection",
        }
    }
}

/// Keys accepted in a `[[guardrails]]` entry for each `type` value.
///
/// `type` itself is always permitted; the second element lists the extra keys
/// that type takes. Consumed by `config::validate_guardrail_keys`.
pub(super) const GUARDRAIL_KEYS_BY_TYPE: &[(&str, &[&str])] = &[
    ("regex", &["name", "pattern", "stages", "action"]),
    ("cel", &["name", "expression", "stages", "action", "fail_open"]),
    ("length_cap", &["name", "max_chars", "stages"]),
    ("prompt_injection", &["name"]),
];

/// Keys accepted in a `type = "regex"` entry's `action` table for each `kind`.
pub(super) const REGEX_ACTION_KEYS_BY_KIND: &[(&str, &[&str])] =
    &[("block", &["code", "reason_prefix"]), ("redact", &["replacement"])];

/// Keys accepted in a `type = "cel"` entry's `action` table for each `kind`.
pub(super) const CEL_ACTION_KEYS_BY_KIND: &[(&str, &[&str])] =
    &[("block", &["code", "reason"]), ("mutate", &["new_payload"])];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProxyConfig;

    fn parse(toml: &str) -> Result<ProxyConfig, String> {
        ProxyConfig::from_toml_str(toml)
    }

    // ~keep `Result::expect_err` requires the Ok type to be Debug, and `ProxyConfig`
    // ~keep deliberately does not derive it — e5d2f79a0 removed that derive so a virtual
    // ~keep key token could never reach a debug-formatted span or panic message. Matching
    // ~keep on the variant keeps that property instead of restoring the derive for tests.
    fn parse_err(toml: &str, why: &str) -> String {
        match parse(toml) {
            Err(error) => error,
            Ok(_) => panic!("{why}"),
        }
    }

    #[test]
    fn absent_section_yields_no_entries() {
        let config = parse("").expect("empty config should parse");
        assert!(config.guardrails.is_empty());
    }

    #[test]
    fn regex_block_entry_parses() {
        let config = parse(
            r#"
[[guardrails]]
type = "regex"
name = "block-ssn"
pattern = '\d{3}-\d{2}-\d{4}'
stages = ["input", "output"]
action = { kind = "block", code = 1100, reason_prefix = "SSN detected" }
"#,
        )
        .expect("regex guardrail should parse");

        assert_eq!(config.guardrails.len(), 1);
        match &config.guardrails[0] {
            GuardrailEntry::Regex {
                name,
                stages,
                action,
                pattern,
            } => {
                assert_eq!(name, "block-ssn");
                assert_eq!(pattern, r"\d{3}-\d{2}-\d{4}");
                assert_eq!(stages, &[GuardrailStageConfig::Input, GuardrailStageConfig::Output]);
                assert_eq!(
                    action,
                    &RegexActionConfig::Block {
                        code: 1100,
                        reason_prefix: "SSN detected".to_owned(),
                    }
                );
            }
            other => panic!("expected Regex entry, got {other:?}"),
        }
    }

    #[test]
    fn regex_redact_entry_parses() {
        let config = parse(
            r#"
[[guardrails]]
type = "regex"
name = "redact-email"
pattern = '\w+@\w+'
stages = ["output_chunk"]
action = { kind = "redact", replacement = "[REDACTED]" }
"#,
        )
        .expect("redact guardrail should parse");

        match &config.guardrails[0] {
            GuardrailEntry::Regex { action, stages, .. } => {
                assert_eq!(stages, &[GuardrailStageConfig::OutputChunk]);
                assert_eq!(
                    action,
                    &RegexActionConfig::Redact {
                        replacement: "[REDACTED]".to_owned(),
                    }
                );
            }
            other => panic!("expected Regex entry, got {other:?}"),
        }
    }

    #[test]
    fn all_entry_types_parse_together() {
        let config = parse(
            r#"
[[guardrails]]
type = "length_cap"
name = "cap"
max_chars = 1000
stages = ["input"]

[[guardrails]]
type = "prompt_injection"
name = "injection"
"#,
        )
        .expect("all builtin guardrail types should parse");

        let labels: Vec<&str> = config.guardrails.iter().map(GuardrailEntry::type_label).collect();
        assert_eq!(labels, ["length_cap", "prompt_injection"]);
        let names: Vec<&str> = config.guardrails.iter().map(GuardrailEntry::name).collect();
        assert_eq!(names, ["cap", "injection"]);
    }

    /// The metadata-driven builtins must stay out of the TOML surface until
    /// `GuardrailContext::metadata` is populated per call — see the module
    /// docs. Accepting them would ship a `deny_list` that never blocks.
    #[test]
    fn metadata_driven_types_are_not_exposed() {
        for type_name in ["deny_list", "allow_list"] {
            let error = parse_err(
                &format!(
                    r#"
[[guardrails]]
type = "{type_name}"
name = "x"
field = "tenant_id"
values = ["y"]
"#
                ),
                "metadata-driven guardrails must not be configurable yet",
            );
            assert!(
                error.contains(type_name) || error.contains("unknown variant"),
                "error should reject {type_name}: {error}"
            );
        }
    }

    #[test]
    fn cel_entry_parses_and_defaults_to_fail_closed() {
        let config = parse(
            r#"
[[guardrails]]
type = "cel"
name = "cel-check"
expression = 'true'
stages = ["input"]
action = { kind = "block", code = 1200, reason = "nope" }
"#,
        )
        .expect("cel guardrail should parse");

        match &config.guardrails[0] {
            GuardrailEntry::Cel { fail_open, action, .. } => {
                assert!(!fail_open, "fail_open must default to false (fail closed)");
                assert_eq!(
                    action,
                    &CelActionConfig::Block {
                        code: 1200,
                        reason: "nope".to_owned(),
                    }
                );
            }
            other => panic!("expected Cel entry, got {other:?}"),
        }
    }

    #[test]
    fn unknown_type_is_rejected() {
        let error = parse_err(
            r#"
[[guardrails]]
type = "regx"
name = "typo"
"#,
            "an unrecognised type must not parse",
        );
        assert!(
            error.contains("regx") || error.contains("unknown variant"),
            "error should name the bad type, got: {error}"
        );
    }

    #[test]
    fn missing_stages_is_rejected() {
        let error = parse_err(
            r#"
[[guardrails]]
type = "regex"
name = "no-stages"
pattern = 'x'
action = { kind = "redact", replacement = "y" }
"#,
            "stages has no default and must be required",
        );
        assert!(error.contains("stages"), "error should name the missing field: {error}");
    }

    #[test]
    fn misspelled_entry_key_is_rejected() {
        let error = parse_err(
            r#"
[[guardrails]]
type = "regex"
name = "typo"
patern = 'x'
stages = ["input"]
action = { kind = "redact", replacement = "y" }
"#,
            "a misspelled key must not be silently discarded",
        );
        assert!(error.contains("patern"), "error should name the unknown key: {error}");
    }

    #[test]
    fn misspelled_action_key_is_rejected() {
        let error = parse_err(
            r#"
[[guardrails]]
type = "regex"
name = "typo"
pattern = 'x'
stages = ["input"]
action = { kind = "redact", replacment = "y" }
"#,
            "a misspelled action key must not be silently discarded",
        );
        assert!(
            error.contains("replacment"),
            "error should name the unknown action key: {error}"
        );
    }

    #[test]
    fn unknown_stage_value_is_rejected() {
        let error = parse_err(
            r#"
[[guardrails]]
type = "regex"
name = "bad-stage"
pattern = 'x'
stages = ["inpt"]
action = { kind = "redact", replacement = "y" }
"#,
            "an unrecognised stage must not parse",
        );
        assert!(
            error.contains("inpt") || error.contains("unknown variant"),
            "error should name the bad stage: {error}"
        );
    }
}
