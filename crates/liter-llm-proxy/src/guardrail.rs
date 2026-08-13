//! Builds the proxy's live [`GuardrailRegistry`] from `[[guardrails]]` config.
//!
//! This is the single construction site for the guardrail set. The registry it
//! returns is stored on [`crate::service_pool::ServicePool`], layered into
//! every model's unary Tower stack via `GuardrailLayer`, and handed to
//! [`crate::state::AppState`] for the realtime WebSocket proxy — so both paths
//! enforce the same `Arc`, and cannot drift apart.
//!
//! # Fail-closed contract
//!
//! [`build_registry`] returns `Err` if **any** configured entry cannot be
//! constructed, and never returns a partially-populated registry. Its only
//! caller is [`crate::service_pool::ServicePool::from_config`], whose `Result`
//! is propagated by `ProxyServer::serve_with_shutdown` before the listener is
//! bound — so a rejected guardrail config means the proxy exits without ever
//! accepting a request.
//!
//! This matters because an empty registry is behaviourally identical to no
//! guardrails at all. If a bad entry were skipped with a warning, an operator
//! who configured five guardrails could be running four, or none, and every
//! request would still return `200`. The only safe response to "this guardrail
//! does not build" is to refuse to serve traffic. ~keep

use std::collections::HashSet;
use std::sync::Arc;

use regex::Regex;

use liter_llm::guardrail::builtin::{LengthCapGuardrail, OnMatch, PromptInjectionHeuristic, RegexGuardrail};
use liter_llm::guardrail::{Guardrail, GuardrailRegistry, GuardrailStage};

use crate::config::{GuardrailEntry, GuardrailStageConfig, RegexActionConfig};

/// Promote a config-supplied string to `&'static str`.
///
/// The core [`Guardrail`] trait returns `&'static str` from `name()` so a
/// guardrail's name can be used as a metric label without allocating on the
/// request path, and `supported_stages()` is `&'static` for the same reason.
/// Config supplies owned `String`s, so bridging the two requires either a leak
/// here or a breaking change to a published trait.
///
/// The leak is bounded and startup-only: one allocation per configured
/// guardrail per `ServicePool` build, never one per request. ~keep
fn leak_str(value: &str) -> &'static str {
    Box::leak(value.to_owned().into_boxed_str())
}

/// Promote configured stages to the `&'static [GuardrailStage]` the core trait
/// requires. See [`leak_str`] for why this leaks.
///
/// # Errors
///
/// Returns an error when `stages` is empty. A guardrail with no stages is
/// never invoked by [`GuardrailRegistry::run_stage`], which is precisely the
/// "declared in config, enforced nowhere" state this module exists to prevent
/// — so it is a config error, not an inert entry. ~keep
fn leak_stages(stages: &[GuardrailStageConfig], entry: &str) -> Result<&'static [GuardrailStage], String> {
    if stages.is_empty() {
        return Err(format!(
            "guardrail '{entry}': `stages` is empty, so this guardrail would never run. Remove the entry or list at \
             least one of \"input\", \"output\", \"output_chunk\"."
        ));
    }

    let mapped: Vec<GuardrailStage> = stages
        .iter()
        .map(|stage| match stage {
            GuardrailStageConfig::Input => GuardrailStage::Input,
            GuardrailStageConfig::Output => GuardrailStage::Output,
            GuardrailStageConfig::OutputChunk => GuardrailStage::OutputChunk,
        })
        .collect();

    Ok(Box::leak(mapped.into_boxed_slice()))
}

/// Build one [`Guardrail`] from one config entry.
///
/// # Errors
///
/// Returns a message naming the offending entry when the guardrail cannot be
/// constructed — an invalid regex, an invalid CEL expression, an empty stage
/// list, or a `type = "cel"` entry in a build compiled without the
/// `guardrail-cel` feature.
fn build_one(entry: &GuardrailEntry) -> Result<Arc<dyn Guardrail>, String> {
    let entry_name = entry.name();

    match entry {
        GuardrailEntry::Regex {
            name,
            pattern,
            stages,
            action,
        } => {
            let compiled = Regex::new(pattern)
                .map_err(|e| format!("guardrail '{name}': invalid regex pattern {pattern:?}: {e}"))?;
            let on_match = match action {
                RegexActionConfig::Block { code, reason_prefix } => OnMatch::Block {
                    code: *code,
                    reason_prefix: reason_prefix.clone(),
                },
                RegexActionConfig::Redact { replacement } => OnMatch::Redact {
                    replacement: replacement.clone(),
                },
            };
            Ok(Arc::new(RegexGuardrail::new(
                leak_str(name),
                compiled,
                on_match,
                leak_stages(stages, entry_name)?,
            )))
        }

        GuardrailEntry::LengthCap {
            name,
            max_chars,
            stages,
        } => Ok(Arc::new(LengthCapGuardrail::new(
            leak_str(name),
            *max_chars,
            leak_stages(stages, entry_name)?,
        ))),

        GuardrailEntry::PromptInjection { name } => Ok(Arc::new(PromptInjectionHeuristic::new(leak_str(name)))),

        #[cfg(feature = "guardrail-cel")]
        GuardrailEntry::Cel {
            name,
            expression,
            stages,
            action,
            fail_open,
        } => {
            use liter_llm::guardrail::cel::{CelAction, CelGuardrail};

            use crate::config::CelActionConfig;

            let on_true = match action {
                CelActionConfig::Block { code, reason } => CelAction::Block {
                    code: *code,
                    reason: reason.clone(),
                },
                CelActionConfig::Mutate { new_payload } => CelAction::Mutate {
                    new_payload: new_payload.clone(),
                },
            };

            let guardrail = CelGuardrail::new(leak_str(name), expression, on_true, leak_stages(stages, entry_name)?)
                .map_err(|e| format!("guardrail '{name}': invalid CEL expression: {e}"))?;

            if *fail_open {
                tracing::warn!(
                    guardrail_name = %name,
                    "guardrail is configured fail_open: a CEL evaluation error will ALLOW the payload through"
                );
            }

            Ok(Arc::new(guardrail.with_fail_open(*fail_open)))
        }

        // ~keep A cel entry in a build without the feature is a hard error, never a
        // ~keep skip: the operator wrote a control they believe is running, and the
        // ~keep binary cannot run it. Degrading here would mean shipping the exact
        // ~keep silently-unenforced guardrail this module exists to rule out.
        #[cfg(not(feature = "guardrail-cel"))]
        GuardrailEntry::Cel { name, .. } => Err(format!(
            "guardrail '{name}': type = \"cel\" requires the `guardrail-cel` feature, which this binary was built \
             without. Rebuild liter-llm-proxy with `--features guardrail-cel`, or remove the entry."
        )),
    }
}

/// Build the proxy's guardrail registry from `[[guardrails]]` config entries.
///
/// Entries are registered in declaration order; the first `Block` decision at
/// a given stage short-circuits the rest.
///
/// # Errors
///
/// Returns an error, aborting startup, when any entry cannot be built or when
/// two entries share a `name`. Duplicate names are rejected because the name
/// is the guardrail's metric label and log identifier: two guardrails
/// answering to one name make a "which guardrail blocked this" question
/// unanswerable, and in practice a duplicate is a copy-pasted entry whose
/// body was never edited.
#[tracing::instrument(level = "debug", skip_all, fields(configured = entries.len()))]
pub fn build_registry(entries: &[GuardrailEntry]) -> Result<Arc<GuardrailRegistry>, String> {
    let mut registry = GuardrailRegistry::new();
    let mut seen: HashSet<&str> = HashSet::with_capacity(entries.len());

    for entry in entries {
        let name = entry.name();
        if name.trim().is_empty() {
            return Err(format!(
                "guardrail of type \"{}\": `name` is empty. Every guardrail needs a name; it is the identifier used \
                 in logs and metrics.",
                entry.type_label()
            ));
        }
        if !seen.insert(name) {
            return Err(format!(
                "duplicate guardrail name '{name}'. Guardrail names must be unique — they are used as metric labels \
                 and in block diagnostics."
            ));
        }
        registry.register(build_one(entry)?);
    }

    // ~keep A non-empty config that produced an empty registry would be the exact
    // ~keep failure this module rules out, wearing the disguise of success. It is
    // ~keep unreachable given the loop above, so treat it as the code bug it would
    // ~keep be rather than serving traffic unguarded.
    if !entries.is_empty() && registry.is_empty() {
        return Err(format!(
            "internal error: {} guardrail(s) configured but none were registered; refusing to start unguarded",
            entries.len()
        ));
    }

    if registry.is_empty() {
        tracing::debug!("no guardrails configured");
    } else {
        tracing::info!(count = registry.len(), "guardrails enabled");
    }

    Ok(Arc::new(registry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProxyConfig;

    fn entries(toml: &str) -> Vec<GuardrailEntry> {
        ProxyConfig::from_toml_str(toml)
            .expect("config fixture should parse")
            .guardrails
    }

    #[test]
    fn absent_config_builds_an_empty_registry() {
        let registry = build_registry(&[]).expect("no guardrails should build");
        assert!(registry.is_empty());
    }

    #[test]
    fn every_builtin_type_builds() {
        let configured = entries(
            r#"
[[guardrails]]
type = "regex"
name = "ssn"
pattern = '\d{3}-\d{2}-\d{4}'
stages = ["input", "output"]
action = { kind = "block", code = 1100, reason_prefix = "SSN" }

[[guardrails]]
type = "length_cap"
name = "cap"
max_chars = 100
stages = ["input"]

[[guardrails]]
type = "prompt_injection"
name = "injection"
"#,
        );

        let registry = build_registry(&configured).expect("all builtin guardrails should build");
        assert_eq!(registry.len(), 3);
        let names: Vec<&str> = registry.iter().map(|g| g.name()).collect();
        assert_eq!(
            names,
            ["ssn", "cap", "injection"],
            "declaration order must be preserved"
        );
    }

    #[test]
    fn configured_stages_reach_the_built_guardrail() {
        let configured = entries(
            r#"
[[guardrails]]
type = "length_cap"
name = "cap"
max_chars = 100
stages = ["output", "output_chunk"]
"#,
        );

        let registry = build_registry(&configured).expect("length cap should build");
        let guardrail = registry.iter().next().expect("one guardrail");
        assert_eq!(
            guardrail.supported_stages(),
            &[GuardrailStage::Output, GuardrailStage::OutputChunk]
        );
    }

    #[test]
    fn invalid_regex_fails_the_whole_build() {
        let configured = entries(
            r#"
[[guardrails]]
type = "regex"
name = "broken"
pattern = '([unclosed'
stages = ["input"]
action = { kind = "redact", replacement = "x" }
"#,
        );

        let error = build_registry(&configured).expect_err("an invalid regex must abort the build");
        assert!(error.contains("broken"), "error should name the entry: {error}");
        assert!(error.contains("invalid regex"), "error should say why: {error}");
    }

    #[test]
    fn one_invalid_entry_rejects_the_valid_entries_with_it() {
        let configured = entries(
            r#"
[[guardrails]]
type = "prompt_injection"
name = "good"

[[guardrails]]
type = "regex"
name = "broken"
pattern = '([unclosed'
stages = ["input"]
action = { kind = "redact", replacement = "x" }
"#,
        );

        assert!(
            build_registry(&configured).is_err(),
            "a partial registry is never returned — one bad entry rejects the set"
        );
    }

    #[test]
    fn empty_stages_are_rejected() {
        let configured = entries(
            r#"
[[guardrails]]
type = "length_cap"
name = "never-runs"
max_chars = 10
stages = []
"#,
        );

        let error = build_registry(&configured).expect_err("a guardrail that never runs must be rejected");
        assert!(error.contains("never-runs"), "error should name the entry: {error}");
        assert!(error.contains("stages"), "error should name the field: {error}");
    }

    #[test]
    fn duplicate_names_are_rejected() {
        let configured = entries(
            r#"
[[guardrails]]
type = "prompt_injection"
name = "dupe"

[[guardrails]]
type = "prompt_injection"
name = "dupe"
"#,
        );

        let error = build_registry(&configured).expect_err("duplicate names must be rejected");
        assert!(error.contains("dupe"), "error should name the collision: {error}");
    }

    #[test]
    fn empty_name_is_rejected() {
        let configured = entries(
            r#"
[[guardrails]]
type = "prompt_injection"
name = "  "
"#,
        );

        assert!(build_registry(&configured).is_err(), "a blank name must be rejected");
    }

    /// A `type = "cel"` entry must never be silently dropped. With the feature
    /// on it compiles (and a bad expression aborts the build); with the feature
    /// off it is a startup error naming the missing feature.
    #[test]
    fn cel_entry_is_never_silently_skipped() {
        let valid = entries(
            r#"
[[guardrails]]
type = "cel"
name = "cel-ok"
expression = 'true'
stages = ["input"]
action = { kind = "block", code = 1200, reason = "nope" }
"#,
        );

        let result = build_registry(&valid);

        #[cfg(feature = "guardrail-cel")]
        {
            let registry = result.expect("a valid CEL expression should build with the feature on");
            assert_eq!(registry.len(), 1, "the CEL guardrail must be registered, not skipped");
        }

        #[cfg(not(feature = "guardrail-cel"))]
        {
            let error = result.expect_err("a CEL entry without the feature must abort startup");
            assert!(
                error.contains("guardrail-cel"),
                "error should name the required feature: {error}"
            );
        }
    }

    #[cfg(feature = "guardrail-cel")]
    #[test]
    fn invalid_cel_expression_fails_the_build() {
        let configured = entries(
            r#"
[[guardrails]]
type = "cel"
name = "cel-broken"
expression = 'this is (not cel'
stages = ["input"]
action = { kind = "block", code = 1200, reason = "nope" }
"#,
        );

        let error = build_registry(&configured).expect_err("an invalid CEL expression must abort the build");
        assert!(error.contains("cel-broken"), "error should name the entry: {error}");
    }
}
