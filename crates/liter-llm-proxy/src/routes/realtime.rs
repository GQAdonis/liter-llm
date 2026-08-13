//! WebSocket proxy for the OpenAI Realtime API.
//!
//! # Route
//!
//! `GET /v1/realtime?model=<model>` — upgrades the HTTP connection to a
//! WebSocket and proxies messages bidirectionally between the API client and
//! the upstream provider's Realtime endpoint.
//!
//! # Message flow
//!
//! ```text
//! client  ──[WS message]──►  translate_outbound  ──►  upstream provider
//! client  ◄─[WS message]──   translate_inbound   ◄──  upstream provider
//! ```
//!
//! # Guardrails
//!
//! [`handle_session`] enforces the guardrail set configured under
//! `[[guardrails]]`, read from [`AppState::guardrails`] via
//! [`session_guardrails`].  It is the *same* [`GuardrailRegistry`] that
//! [`crate::service_pool::ServicePool`] layers into every model's unary Tower
//! stack — one `Arc`, obtained from [`ServicePool::guardrails`], so a
//! guardrail can never be live on one path and absent on the other.
//!
//! Every message is checked: client → upstream at the `GuardrailStage::Input`
//! stage, upstream → client at `GuardrailStage::OutputChunk`.  A `Block`
//! decision is returned to the client as an error event and the message is not
//! forwarded; a `Mutate` decision rewrites the payload that is forwarded.
//!
//! Note that stage semantics differ between the two paths: the unary path
//! inspects a serialized `LlmRequest`, while here the inspected payload is a
//! single realtime event.  A pattern-based guardrail works on both, but a CEL
//! expression written against a chat-completion shape will not evaluate
//! against a realtime event — and `CelGuardrail` fails closed by default, so
//! that misconfiguration blocks realtime traffic loudly rather than passing it
//! silently.
//!
//! [`AppState`]: crate::state::AppState
//! [`AppState::guardrails`]: crate::state::AppState::guardrails
//! [`GuardrailRegistry`]: liter_llm::guardrail::GuardrailRegistry
//! [`ServicePool::guardrails`]: crate::service_pool::ServicePool::guardrails
//!
//! # Cancellation
//!
//! When the client disconnects the upstream WebSocket is closed within
//! one event-loop iteration via a [`tokio_util::sync::CancellationToken`].
//! When [`crate::shutdown::ShutdownHandle`] is draining the same token is
//! cancelled, closing all active sessions.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use axum::Extension;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_util::sync::CancellationToken;

use liter_llm::guardrail::{Guardrail, GuardrailContext, GuardrailDecision, GuardrailStage};
use liter_llm::realtime::{RealtimeEvent, RealtimeTranslator};
use liter_llm::tenant::TenantId;
use liter_llm::tower::TENANT_ID_METADATA_KEY;
use liter_llm::tower::metrics::{record_realtime_bytes, record_realtime_event, record_realtime_session_duration};
use secrecy::{ExposeSecret, SecretString};

use crate::auth::KeyContext;
use crate::config::VirtualKeyConfig;
use crate::error::ProxyError;
use crate::shutdown::ShutdownHandle;
use crate::state::AppState;

/// Query parameters accepted by `GET /v1/realtime`.
#[derive(Debug, Deserialize)]
pub struct RealtimeQueryParams {
    /// The model to use for the realtime session (e.g. `gpt-4o-realtime-preview`).
    pub model: Option<String>,
}

/// `GET /v1/realtime` — upgrades to WebSocket and starts the bidirectional proxy.
///
/// The handler is intentionally thin: it resolves the upstream URL from the
/// configured model and delegates the actual proxying to [`run_proxy`].
///
/// # Security
///
/// - `KeyContext` is extracted from request extensions (populated by the
///   `validate_api_key` middleware) and checked against the requested model
///   **before** the WebSocket upgrade.  A 403 is returned immediately if the
///   virtual key is not allowed to access the model — the upgrade never
///   completes.
/// - The upstream credential is resolved from the virtual key's
///   `provider_credentials` list, **not** from the global `master_key`.  If
///   no matching credential is found for the `"openai"` provider (or for a
///   master-key caller without an explicit credential), the handler returns
///   503 rather than falling back to the master key.
///
/// # Rate limit and budget enforcement
///
/// Unlike unary endpoints, a realtime session never runs through
/// `ServicePool::get_service`'s Tower stack (there is no discrete
/// `LlmRequest`/`LlmResponse` per WebSocket message), so `KeyLimitLayer` and
/// `BudgetLedgerLayer` never see it. [`ServicePool::check_realtime_session_start`]
/// closes that gap at session establishment — **before** the upgrade — reusing
/// the SAME per-key rpm window and budget ledger unary calls use, rather than
/// a parallel realtime-only implementation. See that method's docs for what
/// it can and cannot enforce (it cannot meter the live session's own token
/// usage, only gate on rpm and already-recorded spend).
pub async fn realtime_websocket(
    ws: WebSocketUpgrade,
    Query(params): Query<RealtimeQueryParams>,
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
) -> impl IntoResponse {
    let model = params.model.unwrap_or_default();

    if !key_ctx.can_access_model(&model) {
        let err = ProxyError::forbidden(format!(
            "key '{}' is not allowed to access model '{model}'",
            key_ctx.redacted_id()
        ));
        return err.into_response();
    }

    if let Err(err) = state
        .service_pool
        .check_realtime_session_start(&key_ctx.tenant_id, &model)
        .await
    {
        return err.into_response();
    }

    // ~keep Each request uses one stable config snapshot even if hot-reload fires.
    let config = state.config.load();

    // ~keep Never fall back to master_key; that would let VK holders use the master billing key.
    let upstream_api_key: SecretString = match resolve_upstream_credential(&key_ctx, &config.keys, &model) {
        Some(key) => key,
        None => {
            let err = ProxyError::service_unavailable(format!(
                "no provider credential configured for model '{model}' — \
                 add [[keys.provider_credentials]] with provider = \"openai\" \
                 to the virtual key configuration"
            ));
            return err.into_response();
        }
    };

    let tenant_id = key_ctx.tenant_id.clone();

    ws.on_upgrade(move |socket| handle_session(socket, model, upstream_api_key, state, tenant_id))
        .into_response()
}

/// Resolve an upstream API key from the virtual key's provider credential pool.
///
/// Selection order:
/// 1. Any `provider_credentials` entry with `provider == "openai"` whose
///    `model_allowlist` includes `model` (or whose `model_allowlist` is `None`).
/// 2. First such entry when multiple match (callers should configure one per
///    model group or leave `model_allowlist` unset for a universal credential).
///
/// Returns `None` when:
/// - The caller authenticated as the master key (no VK config) and no explicit
///   credential is provided — returning `None` forces a 503 rather than leaking
///   `master_key` to the upstream.
/// - The VK has no `provider_credentials` entries for `"openai"`.
fn resolve_upstream_credential(
    key_ctx: &KeyContext,
    vk_configs: &[VirtualKeyConfig],
    model: &str,
) -> Option<SecretString> {
    // ~keep Master-key callers need explicit provider credentials; never leak master_key upstream.
    if key_ctx.is_master {
        return None;
    }

    let vk_config = vk_configs.iter().find(|vk| vk.key == key_ctx.key_id)?;

    vk_config
        .provider_credentials
        .iter()
        .find(|cred| {
            cred.provider == "openai"
                && match &cred.model_allowlist {
                    None => true,
                    Some(allowed) => allowed.iter().any(|m| m == model),
                }
        })
        .map(|cred| cred.api_key.clone())
}

/// Derive the cancellation token for one realtime session.
///
/// MUST return a CHILD of the shutdown token, never a clone. `run_proxy`
/// cancels whatever token it is given as soon as either relay loop exits —
/// that is every normal session close — and a clone shares cancellation state
/// with the process-wide token that
/// `axum::serve(..).with_graceful_shutdown(..)` awaits. Cloning here meant one
/// client hanging up a voice session shut down the entire proxy. A child is
/// still cancelled BY the parent, so draining continues to close live
/// sessions, but cancelling the child leaves the parent untouched. ~keep
fn session_cancel_token(shutdown: Option<&ShutdownHandle>) -> CancellationToken {
    shutdown.map_or_else(CancellationToken::default, |handle| {
        handle.cancellation_token().child_token()
    })
}

/// Spawned per WebSocket connection.  Opens the upstream connection using the
/// pre-resolved `upstream_api_key` and runs the bidirectional proxy until
/// either side closes.
///
/// The `upstream_api_key` is the resolved per-VK credential — the master key
/// is NEVER passed here (that check lives in [`realtime_websocket`]).
async fn handle_session(
    client_socket: WebSocket,
    model: String,
    upstream_api_key: SecretString,
    state: AppState,
    tenant_id: TenantId,
) {
    let session_start = Instant::now();

    let guardrail_metadata = session_guardrail_metadata(&tenant_id);

    let encoded_model: String = model
        .chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_' | '~') {
                vec![c]
            } else {
                format!("%{:02X}", c as u32).chars().collect()
            }
        })
        .collect();
    let upstream_url = format!("wss://api.openai.com/v1/realtime?model={encoded_model}");

    tracing::info!(
        model = %model,
        upstream_url = %upstream_url,
        "realtime session starting"
    );

    let upstream = match connect_upstream(&upstream_url, upstream_api_key.expose_secret()).await {
        Ok(ws) => ws,
        Err(err) => {
            tracing::warn!(error = %err, "failed to connect to upstream realtime endpoint");
            send_error_to_axum_socket(client_socket, "upstream_connection_failed", &err.to_string()).await;
            return;
        }
    };

    let cancel = session_cancel_token(state.shutdown.as_ref());

    run_proxy(
        client_socket,
        upstream,
        cancel,
        session_guardrails(&state),
        guardrail_metadata,
        "openai",
    )
    .await;

    let duration = session_start.elapsed().as_secs_f64();
    record_realtime_session_duration("openai", duration);
    tracing::info!(duration_secs = duration, "realtime session ended");
}

/// The guardrail set a realtime session enforces.
///
/// Taken from [`AppState::guardrails`] — the registry
/// [`crate::service_pool::ServicePool`] built at startup and layers into every
/// model's unary Tower stack — never rebuilt here, so realtime and unary
/// traffic are guarded by the same configured set.
///
/// [`run_proxy`] wants a `Vec` because it evaluates guardrails directly rather
/// than through `GuardrailRegistry::run_stage`: realtime frames are neither an
/// `LlmRequest` nor an `LlmResponse`, so the Tower layer does not apply.
///
/// [`AppState::guardrails`]: crate::state::AppState::guardrails
fn session_guardrails(state: &AppState) -> Vec<Arc<dyn Guardrail>> {
    state.guardrails.iter().map(Arc::clone).collect()
}

type UpstreamStream = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Connect to the upstream WebSocket endpoint using the pre-resolved API key.
///
/// # Security
///
/// `api_key` must be the per-VK credential resolved in [`realtime_websocket`].
/// This function must NEVER be called with the global `master_key` — that
/// check is enforced at the call-site in [`handle_session`] which receives
/// the key from [`realtime_websocket`] (never from `AppState.config`).
async fn connect_upstream(url: &str, api_key: &str) -> Result<UpstreamStream, String> {
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::http::Request;

    let request = Request::builder()
        .uri(url)
        .header("User-Agent", "liter-llm-proxy/realtime")
        .header("Authorization", format!("Bearer {api_key}"))
        .body(())
        .map_err(|e| format!("failed to build upstream request: {e}"))?;

    let (ws, _response) = connect_async(request)
        .await
        .map_err(|e| format!("upstream WebSocket handshake failed: {e}"))?;

    Ok(ws)
}

/// The `GuardrailContext::metadata` a realtime session presents to its guardrails.
///
/// The unary path gets this for free — `GuardrailService` derives it from
/// [`liter_llm::tower::types::LlmRequest::tenant_id`] on every call. The relay proxies raw
/// frames and has no `LlmRequest` to derive one from, so it must assemble the equivalent
/// map itself. Without it, a `deny_list` on `tenant_id` would block unary traffic and
/// silently allow realtime traffic: one configured set enforcing differently depending on
/// which door the request came through, which is the failure the shared registry exists to
/// prevent.
///
/// Uses [`TENANT_ID_METADATA_KEY`] rather than a local `"tenant_id"` literal so the two
/// sides cannot drift into different spellings. ~keep
fn session_guardrail_metadata(tenant_id: &TenantId) -> HashMap<String, String> {
    HashMap::from([(TENANT_ID_METADATA_KEY.to_owned(), tenant_id.as_ref().to_owned())])
}

/// Run the bidirectional proxy loop.
///
/// - Reads from `client_socket` (axum `WebSocket`) and forwards to `upstream`
///   (tungstenite `WebSocketStream`) after applying outbound guardrails.
/// - Reads from `upstream` and forwards to `client_socket` after applying
///   inbound guardrails.
/// - Returns when either side closes or `cancel` is triggered.
pub(crate) async fn run_proxy(
    client_socket: WebSocket,
    upstream: UpstreamStream,
    cancel: CancellationToken,
    guardrails: Vec<Arc<dyn Guardrail>>,
    metadata: HashMap<String, String>,
    provider: &'static str,
) {
    let translator = liter_llm::realtime::OpenAiRealtimeTranslator::new();

    let (client_tx, client_rx) = client_socket.split();
    let (upstream_tx, upstream_rx) = upstream.split();

    let client_tx = Arc::new(Mutex::new(client_tx));
    let upstream_tx = Arc::new(Mutex::new(upstream_tx));

    let guardrails = Arc::new(guardrails);

    let upstream_tx_c2u = Arc::clone(&upstream_tx);
    let client_tx_c2u = Arc::clone(&client_tx);
    let cancel_c2u = cancel.clone();
    let metadata = Arc::new(metadata);
    let guardrails_c2u = Arc::clone(&guardrails);
    let metadata_c2u = Arc::clone(&metadata);
    let translator_c2u = translator.clone();

    let c2u = tokio::spawn(async move {
        let mut stream = client_rx;
        loop {
            tokio::select! {
                biased;
                _ = cancel_c2u.cancelled() => break,
                msg = stream.next() => {
                    match msg {
                        None => break,
                        Some(Err(e)) => {
                            tracing::debug!(error = %e, "client socket error");
                            break;
                        }
                        Some(Ok(Message::Close(_))) => break,
                        Some(Ok(Message::Text(text))) => {
                            let raw: serde_json::Value = match serde_json::from_str(&text) {
                                Ok(v) => v,
                                Err(e) => {
                                    tracing::debug!(error = %e, "client sent invalid JSON");
                                    continue;
                                }
                            };

                            let raw = match apply_guardrails_input(&guardrails_c2u, &raw, &metadata_c2u).await {
                                GuardrailOutcome::Allow(payload) => payload,
                                GuardrailOutcome::Block { reason, code } => {
                                    let err_json = guardrail_error_json(code, &reason);
                                    let mut tx = client_tx_c2u.lock().await;
                                    let _ = tx.send(Message::Text(err_json.into())).await;
                                    continue;
                                }
                            };

                            let event = match translator_c2u.translate_inbound(raw) {
                                Ok(e) => e,
                                Err(e) => {
                                    tracing::debug!(error = %e, "translate_inbound failed (c2u)");
                                    continue;
                                }
                            };

                            let label = event_type_label(&event);

                            let audio_bytes = audio_bytes_for_event(&event);

                            let outbound = match translator_c2u.translate_outbound(&event) {
                                Ok(v) => v,
                                Err(e) => {
                                    tracing::debug!(error = %e, "translate_outbound failed");
                                    continue;
                                }
                            };

                            let wire = serde_json::to_string(&outbound).unwrap_or_default();

                            record_realtime_event(provider, "outbound", label);
                            if audio_bytes > 0 {
                                record_realtime_bytes(provider, "outbound", audio_bytes);
                            }

                            let mut tx = upstream_tx_c2u.lock().await;
                            if tx.send(TungsteniteMessage::Text(wire.into())).await.is_err() {
                                break;
                            }
                        }
                        Some(Ok(Message::Binary(bytes))) => {
                            record_realtime_event(provider, "outbound", "binary");
                            record_realtime_bytes(provider, "outbound", bytes.len() as u64);
                            let mut tx = upstream_tx_c2u.lock().await;
                            if tx
                                .send(TungsteniteMessage::Binary(bytes.to_vec().into()))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let mut tx = upstream_tx_c2u.lock().await;
                            let _ = tx.send(TungsteniteMessage::Ping(data.to_vec().into())).await;
                        }
                        Some(Ok(Message::Pong(_))) => {}
                    }
                }
            }
        }
        cancel_c2u.cancel();
    });

    let client_tx_u2c = Arc::clone(&client_tx);
    let cancel_u2c = cancel.clone();
    let guardrails_u2c = Arc::clone(&guardrails);
    let metadata_u2c = Arc::clone(&metadata);
    let translator_u2c = translator;

    let u2c = tokio::spawn(async move {
        let mut stream = upstream_rx;
        loop {
            tokio::select! {
                biased;
                _ = cancel_u2c.cancelled() => break,
                msg = stream.next() => {
                    match msg {
                        None => break,
                        Some(Err(e)) => {
                            tracing::debug!(error = %e, "upstream socket error");
                            break;
                        }
                        Some(Ok(TungsteniteMessage::Text(text))) => {
                            let raw: serde_json::Value =
                                match serde_json::from_str(text.as_str()) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        tracing::debug!(
                                            error = %e,
                                            "upstream sent invalid JSON"
                                        );
                                        let mut tx = client_tx_u2c.lock().await;
                                        let _ = tx
                                            .send(Message::Text(text.to_string().into()))
                                            .await;
                                        continue;
                                    }
                                };

                            let event = match translator_u2c.translate_inbound(raw.clone()) {
                                Ok(e) => e,
                                Err(e) => {
                                    tracing::debug!(
                                        error = %e,
                                        "translate_inbound failed (u2c)"
                                    );
                                    continue;
                                }
                            };

                            let label = event_type_label(&event);
                            let audio_bytes = audio_bytes_for_event(&event);

                            let forward_json = match apply_guardrails_output_chunk(
                                &guardrails_u2c,
                                &raw,
                                &metadata_u2c,
                            )
                            .await
                            {
                                GuardrailOutcome::Allow(v) => {
                                    serde_json::to_string(&v).unwrap_or_default()
                                }
                                GuardrailOutcome::Block { reason, code } => {
                                    let err = RealtimeEvent::Error {
                                        code: format!("{code}"),
                                        message: reason,
                                        event_id: None,
                                    };
                                    match translator_u2c.translate_outbound(&err) {
                                        Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
                                        Err(_) => continue,
                                    }
                                }
                            };

                            record_realtime_event(provider, "inbound", label);
                            if audio_bytes > 0 {
                                record_realtime_bytes(provider, "inbound", audio_bytes);
                            }

                            let mut tx = client_tx_u2c.lock().await;
                            if tx.send(Message::Text(forward_json.into())).await.is_err() {
                                break;
                            }
                        }
                        Some(Ok(TungsteniteMessage::Binary(bytes))) => {
                            record_realtime_event(provider, "inbound", "binary");
                            record_realtime_bytes(provider, "inbound", bytes.len() as u64);
                            let mut tx = client_tx_u2c.lock().await;
                            if tx
                                .send(Message::Binary(bytes.to_vec().into()))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                        Some(Ok(TungsteniteMessage::Ping(data))) => {
                            let mut tx = client_tx_u2c.lock().await;
                            let _ = tx.send(Message::Ping(data.to_vec().into())).await;
                        }
                        Some(Ok(TungsteniteMessage::Close(_))) => break,
                        Some(Ok(
                            TungsteniteMessage::Pong(_) | TungsteniteMessage::Frame(_),
                        )) => {}
                    }
                }
            }
        }
        cancel_u2c.cancel();
    });

    let _ = tokio::join!(c2u, u2c);
}

/// Outcome of running guardrails on a payload.
pub(crate) enum GuardrailOutcome {
    /// Forward this payload — the original, or the rewrite a `Mutate`
    /// decision produced.
    Allow(serde_json::Value),
    /// Do not forward; return an error event to the client instead.
    Block { reason: String, code: u32 },
}

/// Render a guardrail `Block` as the realtime error event sent to the client.
pub(crate) fn guardrail_error_json(code: u32, reason: &str) -> String {
    let err = serde_json::json!({
        "type": "error",
        "error": {
            "code": format!("{code}"),
            "message": reason,
        }
    });
    serde_json::to_string(&err).unwrap_or_default()
}

/// Run the `Input` stage over a client → upstream event.
///
/// A `Mutate` decision rewrites the payload and evaluation continues against
/// the rewritten value, so the next guardrail inspects what will actually be
/// forwarded and a redaction guardrail redacts here exactly as it does on the
/// unary path. Dropping the rewrite and forwarding the original — as this
/// function did while no caller supplied a guardrail set — is how a redaction
/// guardrail comes to leak the content it was installed to remove. ~keep
pub(crate) async fn apply_guardrails_input(
    guardrails: &[Arc<dyn Guardrail>],
    payload: &serde_json::Value,
    metadata: &HashMap<String, String>,
) -> GuardrailOutcome {
    let mut current = payload.clone();

    for guardrail in guardrails.iter() {
        if !guardrail.supported_stages().contains(&GuardrailStage::Input) {
            continue;
        }
        let ctx = GuardrailContext {
            request: &current,
            response: None,
            chunk: None,
            metadata,
        };
        match guardrail.check(GuardrailStage::Input, &ctx).await {
            GuardrailDecision::Block { reason, code } => {
                return GuardrailOutcome::Block { reason, code };
            }
            GuardrailDecision::Mutate { new_payload } => {
                current = new_payload;
            }
            GuardrailDecision::Allow => {}
        }
    }

    GuardrailOutcome::Allow(current)
}

pub(crate) async fn apply_guardrails_output_chunk(
    guardrails: &[Arc<dyn Guardrail>],
    payload: &serde_json::Value,
    metadata: &HashMap<String, String>,
) -> GuardrailOutcome {
    let payload_str = serde_json::to_string(payload).unwrap_or_default();
    let mut current = payload.clone();

    for guardrail in guardrails.iter() {
        if !guardrail.supported_stages().contains(&GuardrailStage::OutputChunk) {
            continue;
        }
        let ctx = GuardrailContext {
            request: &current,
            response: None,
            chunk: Some(&payload_str),
            metadata,
        };
        match guardrail.check(GuardrailStage::OutputChunk, &ctx).await {
            GuardrailDecision::Block { reason, code } => {
                return GuardrailOutcome::Block { reason, code };
            }
            GuardrailDecision::Mutate { new_payload } => {
                current = new_payload;
            }
            GuardrailDecision::Allow => {}
        }
    }
    GuardrailOutcome::Allow(current)
}

pub(crate) fn event_type_label(event: &RealtimeEvent) -> &'static str {
    match event {
        RealtimeEvent::SessionCreated { .. } => "session.created",
        RealtimeEvent::SessionUpdated { .. } => "session.updated",
        RealtimeEvent::ConversationItemCreated { .. } => "conversation.item.created",
        RealtimeEvent::ConversationItemDeleted { .. } => "conversation.item.deleted",
        RealtimeEvent::ResponseCreated { .. } => "response.created",
        RealtimeEvent::ResponseDone { .. } => "response.done",
        RealtimeEvent::ResponseTextDelta { .. } => "response.text.delta",
        RealtimeEvent::ResponseTextDone { .. } => "response.text.done",
        RealtimeEvent::ResponseAudioDelta { .. } => "response.audio.delta",
        RealtimeEvent::ResponseAudioDone { .. } => "response.audio.done",
        RealtimeEvent::ResponseAudioTranscriptDelta { .. } => "response.audio_transcript.delta",
        RealtimeEvent::ResponseAudioTranscriptDone { .. } => "response.audio_transcript.done",
        RealtimeEvent::ResponseFunctionCallArgumentsDelta { .. } => "response.function_call_arguments.delta",
        RealtimeEvent::ResponseFunctionCallArgumentsDone { .. } => "response.function_call_arguments.done",
        RealtimeEvent::InputAudioBufferAppend { .. } => "input_audio_buffer.append",
        RealtimeEvent::InputAudioBufferCommit => "input_audio_buffer.commit",
        RealtimeEvent::InputAudioBufferClear => "input_audio_buffer.clear",
        RealtimeEvent::InputAudioBufferSpeechStarted { .. } => "input_audio_buffer.speech_started",
        RealtimeEvent::InputAudioBufferSpeechStopped { .. } => "input_audio_buffer.speech_stopped",
        RealtimeEvent::RateLimitsUpdated { .. } => "rate_limits.updated",
        RealtimeEvent::Error { .. } => "error",
        RealtimeEvent::Raw { .. } => "raw",
    }
}

/// Approximate audio byte count for an event (0 when not an audio event).
fn audio_bytes_for_event(event: &RealtimeEvent) -> u64 {
    match event {
        RealtimeEvent::InputAudioBufferAppend { audio_base64 } => (audio_base64.len() * 3 / 4) as u64,
        RealtimeEvent::ResponseAudioDelta { delta_base64, .. } => (delta_base64.len() * 3 / 4) as u64,
        _ => 0,
    }
}

async fn send_error_to_axum_socket(mut socket: WebSocket, code: &str, message: &str) {
    let err = serde_json::json!({
        "type": "error",
        "error": { "code": code, "message": message }
    });
    let _ = socket
        .send(Message::Text(serde_json::to_string(&err).unwrap_or_default().into()))
        .await;
    let _ = socket.close().await;
}

#[cfg(test)]
mod tests {

    /// A deny-list configured on `tenant_id` must decide the same way on both paths. The
    /// unary path gets its metadata from `GuardrailService`; realtime has no `LlmRequest`,
    /// so it assembles the equivalent here. If this map were empty — as it was before —
    /// `DenyListGuardrail` would read `None`, treat it as "nothing to deny", and silently
    /// allow every realtime message while the identical rule blocked unary traffic.
    ///
    /// Revert line: replace `session_guardrail_metadata(&tenant_id)` in `handle_session`
    /// with `HashMap::new()` to make this fail.
    #[test]
    fn session_guardrail_metadata_carries_the_tenant_under_the_core_key() {
        let tenant = TenantId::from("tnt-0123456789abcdef");
        let metadata = session_guardrail_metadata(&tenant);

        assert_eq!(
            metadata.get(TENANT_ID_METADATA_KEY).map(String::as_str),
            Some("tnt-0123456789abcdef"),
            "realtime must present the tenant under the same key the unary path uses"
        );
        assert_eq!(metadata.len(), 1, "only tenant_id is derived for a realtime session");
    }

    /// The key is the core's exported constant, not a local literal, so a rename upstream
    /// cannot leave the two paths spelling it differently.
    #[test]
    fn realtime_metadata_key_matches_the_core_constant() {
        assert_eq!(TENANT_ID_METADATA_KEY, "tenant_id");
    }

    use std::pin::Pin;
    use std::sync::Arc;
    use std::time::Duration;

    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use tokio_tungstenite::tungstenite::Message as Msg;

    use super::*;
    use liter_llm::guardrail::{GuardrailContext, GuardrailDecision, GuardrailStage};
    use liter_llm::realtime::RealtimeEvent;

    async fn spawn_mock_ws_server<F, Fut>(handler: F) -> std::net::SocketAddr
    where
        F: FnOnce(tokio_tungstenite::WebSocketStream<TcpStream>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let ws = accept_async(stream).await.unwrap();
            handler(ws).await;
        });
        addr
    }

    /// Verifies that the mock server can exchange messages bi-directionally.
    ///
    /// The test speaks tungstenite directly (not through axum) because wiring
    /// up a full axum WebSocket inside a unit test requires a real HTTP upgrade
    /// that would turn this into an integration test.  The proxy logic itself
    /// is exercised via the `run_proxy` public function in the integration
    /// companion below; here we focus on the underlying transport plumbing.
    #[tokio::test]
    async fn realtime_websocket_proxy_forwards_bidirectional() {
        let addr = spawn_mock_ws_server(|mut ws| async move {
            let greeting = serde_json::json!({
                "type": "session.created",
                "session": { "id": "sess_1", "model": "gpt-4o-realtime-preview" }
            });
            let _ = ws
                .send(Msg::Text(serde_json::to_string(&greeting).unwrap().into()))
                .await;

            if let Some(Ok(msg)) = ws.next().await {
                let _ = ws.send(msg).await;
            }
        })
        .await;

        let url = format!("ws://{addr}");
        let (mut stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();

        let greeting_msg = stream.next().await.unwrap().unwrap();
        assert!(greeting_msg.is_text(), "expected text frame from upstream");
        let val: serde_json::Value = serde_json::from_str(greeting_msg.into_text().unwrap().as_str()).unwrap();
        assert_eq!(val["type"], "session.created");
        assert_eq!(val["session"]["id"], "sess_1");

        let commit = serde_json::json!({ "type": "input_audio_buffer.commit" });
        stream
            .send(Msg::Text(serde_json::to_string(&commit).unwrap().into()))
            .await
            .unwrap();
        let echo = stream.next().await.unwrap().unwrap();
        assert!(echo.is_text());
        let echo_val: serde_json::Value = serde_json::from_str(echo.into_text().unwrap().as_str()).unwrap();
        assert_eq!(echo_val["type"], "input_audio_buffer.commit");
    }

    /// A pre-cancelled token resolves immediately — models client disconnect.
    #[tokio::test]
    async fn realtime_websocket_proxy_cancels_on_client_disconnect() {
        let cancel = CancellationToken::new();
        cancel.cancel();

        let start = std::time::Instant::now();
        let result = tokio::time::timeout(Duration::from_millis(100), cancel.cancelled()).await;
        assert!(result.is_ok(), "cancellation should complete within 100ms");
        assert!(start.elapsed() < Duration::from_millis(100), "should not have blocked");
    }

    /// Ending one realtime session must NOT shut down the proxy.
    ///
    /// This drives `session_cancel_token` itself rather than re-deriving a
    /// child token inline — an inline version would keep passing if the
    /// production code reverted to `.clone()`, which is precisely the bug.
    #[test]
    fn session_cancel_token_does_not_cancel_the_shutdown_token() {
        let coordinator = crate::shutdown::ShutdownCoordinator::new();
        let handle = coordinator.handle();
        let shutdown = handle.cancellation_token();

        let session = session_cancel_token(Some(&handle));
        // Exactly what run_proxy does when either relay loop exits.
        session.cancel();

        assert!(session.is_cancelled(), "the session token must be cancelled");
        assert!(
            !shutdown.is_cancelled(),
            "a session ending must NOT cancel the process-wide shutdown token — that stops the server"
        );
    }

    /// The session token must still be cancelled BY a real shutdown, or
    /// draining would leave realtime sessions running.
    #[test]
    fn session_cancel_token_is_still_cancelled_by_shutdown() {
        let coordinator = crate::shutdown::ShutdownCoordinator::new();
        let handle = coordinator.handle();
        let session = session_cancel_token(Some(&handle));

        handle.cancellation_token().cancel();

        assert!(
            session.is_cancelled(),
            "drain must still close active realtime sessions"
        );
    }

    /// A guardrail that blocks every event prevents the payload from reaching
    /// the upstream and sends an error event to the client.
    #[tokio::test]
    async fn realtime_websocket_proxy_blocks_on_guardrail() {
        struct BlockAllGuardrail;

        impl Guardrail for BlockAllGuardrail {
            fn name(&self) -> &'static str {
                "block_all"
            }

            fn supported_stages(&self) -> &'static [GuardrailStage] {
                &[GuardrailStage::Input]
            }

            fn check<'a>(
                &'a self,
                _stage: GuardrailStage,
                _ctx: &'a GuardrailContext<'a>,
            ) -> Pin<Box<dyn std::future::Future<Output = GuardrailDecision> + Send + 'a>> {
                Box::pin(async {
                    GuardrailDecision::Block {
                        reason: "blocked by test guardrail".into(),
                        code: 1001,
                    }
                })
            }
        }

        let guardrails: Vec<Arc<dyn Guardrail>> = vec![Arc::new(BlockAllGuardrail)];
        let payload = serde_json::json!({ "type": "input_audio_buffer.commit" });

        let outcome = apply_guardrails_input(&guardrails, &payload, &HashMap::new()).await;

        let GuardrailOutcome::Block { reason, code } = outcome else {
            panic!("guardrail should have blocked the event");
        };
        let err_val: serde_json::Value =
            serde_json::from_str(&guardrail_error_json(code, &reason)).expect("error event should be valid JSON");
        assert_eq!(err_val["type"], "error");
        assert_eq!(err_val["error"]["code"], "1001");
        assert!(
            err_val["error"]["message"]
                .as_str()
                .unwrap_or("")
                .contains("blocked by test guardrail"),
            "error message should mention the guardrail reason"
        );
    }

    #[tokio::test]
    async fn apply_guardrails_output_chunk_allows_clean_event() {
        let guardrails: Vec<Arc<dyn Guardrail>> = vec![];
        let payload = serde_json::json!({ "type": "response.text.delta", "delta": "hello" });
        let outcome = apply_guardrails_output_chunk(&guardrails, &payload, &HashMap::new()).await;
        assert!(matches!(outcome, GuardrailOutcome::Allow(_)));
    }

    /// Build an [`AppState`] the way `ProxyServer::serve_with_shutdown` does —
    /// in particular taking `guardrails` from the pool rather than rebuilding
    /// it. Mirrors `routes::batches::tests::test_state`; no network access
    /// happens at construction time.
    fn test_state(config_toml: &str) -> AppState {
        use arc_swap::ArcSwap;

        use crate::auth::KeyStore;
        use crate::config::{FileStorageConfig, ProxyConfig};
        use crate::file_store::FileStore;
        use crate::secrets::{EnvVarSecretManager, SecretManager, SecretManagerRegistry};
        use crate::service_pool::ServicePool;

        let config = ProxyConfig::from_toml_str(config_toml).expect("valid TOML");
        let service_pool = Arc::new(ServicePool::from_config(&config, None).expect("service pool"));
        let file_store = Arc::new(FileStore::from_config(&FileStorageConfig::default()).expect("file store"));
        let key_store = Arc::new(KeyStore::from_config(None, &[]));
        let secret_registry = Arc::new(
            SecretManagerRegistry::builder()
                .default_backend(Arc::new(EnvVarSecretManager::new()) as Arc<dyn SecretManager>)
                .build(),
        );

        AppState {
            key_store: Arc::clone(&key_store),
            key_resolver: key_store,
            guardrails: service_pool.guardrails(),
            service_pool,
            file_store,
            config: Arc::new(ArcSwap::new(Arc::new(config))),
            secret_registry,
            shutdown: None,
            usage_sink: None,
        }
    }

    const MODEL_TOML: &str = r#"
[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-test"
"#;

    /// A realtime session must enforce the guardrails declared in
    /// `[[guardrails]]`. Before this wiring existed, `handle_session` passed a
    /// hardcoded empty vec and every realtime message went through unexamined.
    #[test]
    fn session_guardrails_returns_the_configured_set() {
        let state = test_state(&format!(
            r#"{MODEL_TOML}
[[guardrails]]
type = "prompt_injection"
name = "injection-heuristic"
"#
        ));

        let guardrails = session_guardrails(&state);

        assert_eq!(guardrails.len(), 1, "the configured guardrail must reach the session");
        assert_eq!(guardrails[0].name(), "injection-heuristic");
    }

    /// The realtime and unary paths must share one registry, not two sets
    /// built from the same config — a shared `Arc` is what makes it impossible
    /// to enable a guardrail on one path only.
    #[test]
    fn session_guardrails_share_the_pool_registry() {
        let state = test_state(&format!(
            r#"{MODEL_TOML}
[[guardrails]]
type = "prompt_injection"
name = "injection-heuristic"
"#
        ));

        assert!(
            Arc::ptr_eq(&state.guardrails, &state.service_pool.guardrails()),
            "AppState must hold the same registry the Tower stacks were built with"
        );
    }

    #[test]
    fn session_guardrails_is_empty_when_none_configured() {
        let state = test_state(MODEL_TOML);
        assert!(
            session_guardrails(&state).is_empty(),
            "an absent [[guardrails]] section must leave realtime behaviour unchanged"
        );
    }

    /// End-to-end for the realtime path: TOML → `build_registry` → the exact
    /// function the client → upstream relay loop calls on every frame.
    #[tokio::test]
    async fn configured_guardrail_blocks_a_realtime_input_event() {
        let config = crate::config::ProxyConfig::from_toml_str(
            r#"
[[guardrails]]
type = "regex"
name = "block-ssn"
pattern = '\d{3}-\d{2}-\d{4}'
stages = ["input"]
action = { kind = "block", code = 1100, reason_prefix = "SSN detected" }
"#,
        )
        .expect("valid TOML");
        let registry = crate::guardrail::build_registry(&config.guardrails).expect("guardrail should build");
        let guardrails: Vec<Arc<dyn Guardrail>> = registry.iter().map(Arc::clone).collect();

        let payload = serde_json::json!({
            "type": "conversation.item.create",
            "item": { "content": [{ "text": "my ssn is 123-45-6789" }] }
        });

        let outcome = apply_guardrails_input(&guardrails, &payload, &HashMap::new()).await;

        let GuardrailOutcome::Block { code, reason } = outcome else {
            panic!("a configured regex guardrail must block a matching realtime event");
        };
        assert_eq!(code, 1100);
        assert!(
            reason.contains("SSN detected"),
            "reason should carry the prefix: {reason}"
        );
    }

    /// A clean event still passes when a guardrail is configured — the check
    /// blocks on a match, it does not reject everything.
    #[tokio::test]
    async fn configured_guardrail_allows_a_clean_realtime_input_event() {
        let config = crate::config::ProxyConfig::from_toml_str(
            r#"
[[guardrails]]
type = "regex"
name = "block-ssn"
pattern = '\d{3}-\d{2}-\d{4}'
stages = ["input"]
action = { kind = "block", code = 1100, reason_prefix = "SSN detected" }
"#,
        )
        .expect("valid TOML");
        let registry = crate::guardrail::build_registry(&config.guardrails).expect("guardrail should build");
        let guardrails: Vec<Arc<dyn Guardrail>> = registry.iter().map(Arc::clone).collect();

        let payload = serde_json::json!({ "type": "input_audio_buffer.commit" });
        let outcome = apply_guardrails_input(&guardrails, &payload, &HashMap::new()).await;

        assert!(matches!(outcome, GuardrailOutcome::Allow(_)));
    }

    /// A `Mutate` decision at the `Input` stage must rewrite the payload that
    /// is forwarded upstream. Dropping the rewrite would make a configured
    /// redaction guardrail forward exactly the content it was installed to
    /// remove.
    #[tokio::test]
    async fn apply_guardrails_input_forwards_the_mutated_payload() {
        struct RedactGuardrail;

        impl Guardrail for RedactGuardrail {
            fn name(&self) -> &'static str {
                "redact_all"
            }

            fn supported_stages(&self) -> &'static [GuardrailStage] {
                &[GuardrailStage::Input]
            }

            fn check<'a>(
                &'a self,
                _stage: GuardrailStage,
                _ctx: &'a GuardrailContext<'a>,
            ) -> Pin<Box<dyn std::future::Future<Output = GuardrailDecision> + Send + 'a>> {
                Box::pin(async {
                    GuardrailDecision::Mutate {
                        new_payload: serde_json::json!({ "type": "redacted" }),
                    }
                })
            }
        }

        let guardrails: Vec<Arc<dyn Guardrail>> = vec![Arc::new(RedactGuardrail)];
        let payload = serde_json::json!({ "type": "conversation.item.create", "secret": "hunter2" });

        let outcome = apply_guardrails_input(&guardrails, &payload, &HashMap::new()).await;

        let GuardrailOutcome::Allow(forwarded) = outcome else {
            panic!("a Mutate decision must allow the rewritten payload through");
        };
        assert_eq!(forwarded, serde_json::json!({ "type": "redacted" }));
        assert!(
            forwarded.get("secret").is_none(),
            "the original payload must not be forwarded after a Mutate decision"
        );
    }

    #[test]
    fn event_type_label_returns_correct_strings() {
        let cases: &[(RealtimeEvent, &str)] = &[
            (RealtimeEvent::InputAudioBufferCommit, "input_audio_buffer.commit"),
            (RealtimeEvent::InputAudioBufferClear, "input_audio_buffer.clear"),
            (
                RealtimeEvent::ResponseCreated {
                    response_id: "r".into(),
                },
                "response.created",
            ),
            (
                RealtimeEvent::Raw {
                    event_type: "x".into(),
                    payload: serde_json::Value::Null,
                },
                "raw",
            ),
        ];
        for (event, expected) in cases {
            assert_eq!(event_type_label(event), *expected);
        }
    }

    #[test]
    fn audio_bytes_for_event_returns_zero_for_non_audio() {
        let event = RealtimeEvent::InputAudioBufferCommit;
        assert_eq!(audio_bytes_for_event(&event), 0);
    }

    #[test]
    fn audio_bytes_for_event_returns_nonzero_for_audio_append() {
        let event = RealtimeEvent::InputAudioBufferAppend {
            audio_base64: "AAAA".into(),
        };
        assert!(audio_bytes_for_event(&event) > 0);
    }

    use secrecy::{ExposeSecret, SecretString};

    use crate::auth::KeyContext;
    use crate::config::VirtualKeyConfig;
    use crate::config::key::ProviderCredential;

    fn make_vk_config(
        key: &str,
        models: Vec<String>,
        provider_credentials: Vec<ProviderCredential>,
    ) -> VirtualKeyConfig {
        VirtualKeyConfig {
            key: key.to_string(),
            tenant_id: None,
            description: None,
            models,
            rpm: None,
            tpm: None,
            budget_limit: None,
            provider_credentials,
        }
    }

    fn make_provider_cred(
        provider: &str,
        id: &str,
        api_key: &str,
        model_allowlist: Option<Vec<String>>,
    ) -> ProviderCredential {
        ProviderCredential {
            provider: provider.to_string(),
            id: id.to_string(),
            api_key: SecretString::from(api_key.to_string()),
            model_allowlist,
        }
    }

    /// `resolve_upstream_credential` returns a `SecretString`.
    /// Verify the key is correctly resolved and the type is `SecretString`.
    #[test]
    fn provider_credential_api_key_is_secret_string() {
        let cred = make_provider_cred("openai", "cred-1", "sk-test-secret", None);
        let _: &SecretString = &cred.api_key;
        let debug = format!("{cred:?}");
        assert!(
            !debug.contains("sk-test-secret"),
            "Debug output must redact the api_key; got: {debug}"
        );
        assert!(
            debug.contains("[REDACTED]"),
            "Debug output must contain '[REDACTED]'; got: {debug}"
        );
    }

    /// `resolve_upstream_credential` must return the per-VK credential for
    /// the matched model, never `master_key`.  Having two VKs with different
    /// OpenAI credentials verifies that the correct one is selected.
    #[test]
    fn realtime_master_key_not_leaked_to_upstream() {
        let cred_a = make_provider_cred("openai", "cred-a", "sk-vk-a-secret", None);
        let cred_b = make_provider_cred("openai", "cred-b", "sk-vk-b-secret", None);

        let vk_a = make_vk_config("vk-team-a", vec!["gpt-4o-realtime".into()], vec![cred_a]);
        let vk_b = make_vk_config("vk-team-b", vec!["gpt-4o-realtime".into()], vec![cred_b]);
        let vk_configs = vec![vk_a, vk_b];

        let ctx_a = KeyContext {
            key_id: "vk-team-a".into(),
            allowed_models: Some(vec!["gpt-4o-realtime".into()]),
            is_master: false,
            tenant_id: liter_llm::tenant::TenantId::from("vk-team-a"),
        };
        let resolved_a = resolve_upstream_credential(&ctx_a, &vk_configs, "gpt-4o-realtime");
        assert_eq!(
            resolved_a.as_ref().map(|s| s.expose_secret()),
            Some("sk-vk-a-secret"),
            "team-a should get its own credential, not master key or team-b's key"
        );

        let ctx_b = KeyContext {
            key_id: "vk-team-b".into(),
            allowed_models: Some(vec!["gpt-4o-realtime".into()]),
            is_master: false,
            tenant_id: liter_llm::tenant::TenantId::from("vk-team-b"),
        };
        let resolved_b = resolve_upstream_credential(&ctx_b, &vk_configs, "gpt-4o-realtime");
        assert_eq!(
            resolved_b.as_ref().map(|s| s.expose_secret()),
            Some("sk-vk-b-secret"),
            "team-b should get its own credential"
        );

        let ctx_master = KeyContext::master();
        let resolved_master = resolve_upstream_credential(&ctx_master, &vk_configs, "gpt-4o-realtime");
        assert!(
            resolved_master.is_none(),
            "master-key caller must never leak master_key to upstream; got Some({resolved_master:?})"
        );
    }

    /// A VK with `model_allowlist = ["gpt-4o-realtime"]` must not receive
    /// a credential when requesting a different model (the request would
    /// already have been rejected at `can_access_model`, but we verify
    /// credential resolution also refuses).
    #[test]
    fn realtime_credential_model_allowlist_respected() {
        let cred = make_provider_cred(
            "openai",
            "cred-1",
            "sk-vk-secret",
            Some(vec!["gpt-4o-realtime-preview".into()]),
        );
        let vk = make_vk_config("vk-1", vec![], vec![cred]);
        let vk_configs = vec![vk];

        let ctx = KeyContext {
            key_id: "vk-1".into(),
            allowed_models: None,
            is_master: false,
            tenant_id: liter_llm::tenant::TenantId::from("vk-1"),
        };

        let matched = resolve_upstream_credential(&ctx, &vk_configs, "gpt-4o-realtime-preview");
        assert_eq!(matched.as_ref().map(|s| s.expose_secret()), Some("sk-vk-secret"));

        let unmatched = resolve_upstream_credential(&ctx, &vk_configs, "gpt-4o-mini");
        assert!(
            unmatched.is_none(),
            "credential with model_allowlist must not be used for an unlisted model"
        );
    }

    /// Verify that `can_access_model` gates model access in realtime.
    /// This tests the `KeyContext` method used by the handler's security gate.
    #[test]
    fn realtime_websocket_denies_unallowed_model_with_403() {
        let ctx = KeyContext {
            key_id: "vk-restricted".into(),
            allowed_models: Some(vec!["gpt-4o".into()]),
            is_master: false,
            tenant_id: liter_llm::tenant::TenantId::from("vk-restricted"),
        };

        assert!(
            !ctx.can_access_model("gpt-4o-mini"),
            "VK restricted to gpt-4o must be denied access to gpt-4o-mini"
        );
        assert!(
            ctx.can_access_model("gpt-4o"),
            "VK restricted to gpt-4o must be allowed access to gpt-4o"
        );
    }
}
