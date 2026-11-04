//! Chat-only terminal contract. Other SSE endpoints retain their own semantics.
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;

use super::{
    request::{ResponseReadOptions, StreamingPost},
    streaming::SseDecoder,
};
use crate::{
    error::{LiterLlmError, Result},
    types::ChatCompletionChunk,
};

/// Terminal semantics supplied by the actual selected provider implementation.
#[derive(Clone, Copy)]
pub(crate) enum ChatStreamProtocol {
    OpenAi,
    Anthropic,
    Gemini,
    Cohere,
}

/// `parse_event` maps one SSE data payload to a chunk; terminal evidence is
/// judged on the raw payload, independently of what the parser keeps.
pub(crate) async fn post_chat_stream<P>(
    client: &reqwest::Client,
    request: StreamingPost<'_>,
    protocol: ChatStreamProtocol,
    provider_name: String,
    parse_event: P,
    options: ResponseReadOptions<'_>,
) -> Result<crate::client::BoxStream<'static, Result<ChatCompletionChunk>>>
where
    P: Fn(&str) -> Result<Option<ChatCompletionChunk>> + Send + 'static,
{
    let response = request.send(client, options).await?;
    Ok(Box::pin(ChatSseParser {
        inner: response.bytes_stream(),
        decoder: SseDecoder::default(),
        parse_event,
        protocol,
        provider_name,
        terminal: false,
        finished: false,
        done: false,
    }))
}

pin_project! {
    struct ChatSseParser<S, P> {
        #[pin]
        inner: S,
        decoder: SseDecoder,
        parse_event: P,
        protocol: ChatStreamProtocol,
        provider_name: String,
        terminal: bool,
        // ~keep OpenAI-compatible only: a chunk carried a finish reason (upstream #356). It does
        // ~keep not end the stream, because a usage chunk may follow it before `[DONE]`.
        finished: bool,
        done: bool,
    }
}

fn failure(message: &str) -> LiterLlmError {
    LiterLlmError::Streaming {
        message: message.into(),
    }
}

impl<S, P> Stream for ChatSseParser<S, P>
where
    S: Stream<Item = std::result::Result<Bytes, reqwest::Error>>,
    P: Fn(&str) -> Result<Option<ChatCompletionChunk>>,
{
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        if *this.done {
            return Poll::Ready(None);
        }
        loop {
            while let Some(event) = this.decoder.next_event() {
                if *this.terminal && !matches!(this.protocol, ChatStreamProtocol::Gemini) {
                    *this.done = true;
                    return Poll::Ready(Some(Err(failure("Provider sent data after the terminal event"))));
                }
                if event.data == "[DONE]" {
                    if matches!(this.protocol, ChatStreamProtocol::OpenAi) {
                        *this.terminal = true;
                        continue;
                    }
                    *this.done = true;
                    return Poll::Ready(Some(Err(failure(
                        "Unexpected terminal sentinel for selected provider protocol",
                    ))));
                }
                let value: serde_json::Value = match serde_json::from_str(&event.data) {
                    Ok(value) => value,
                    Err(_) => {
                        *this.done = true;
                        return Poll::Ready(Some(Err(failure("Malformed provider SSE JSON event"))));
                    }
                };
                let kind = value.get("type").and_then(serde_json::Value::as_str);
                if matches!(
                    this.protocol,
                    ChatStreamProtocol::Anthropic | ChatStreamProtocol::Cohere
                ) && kind.is_none()
                {
                    *this.done = true;
                    return Poll::Ready(Some(Err(failure("Provider SSE event has no type"))));
                }
                if !event.event.is_empty()
                    && matches!(
                        this.protocol,
                        ChatStreamProtocol::Anthropic | ChatStreamProtocol::Cohere
                    )
                    && kind != Some(event.event.as_str())
                {
                    *this.done = true;
                    return Poll::Ready(Some(Err(failure(
                        "Provider SSE event name disagrees with payload type",
                    ))));
                }
                let terminal = match this.protocol {
                    ChatStreamProtocol::OpenAi => false,
                    ChatStreamProtocol::Anthropic => kind == Some("message_stop"),
                    ChatStreamProtocol::Cohere => {
                        kind == Some("message-end")
                            && value
                                .pointer("/delta/finish_reason")
                                .and_then(serde_json::Value::as_str)
                                .is_some_and(|reason| !reason.is_empty())
                    }
                    ChatStreamProtocol::Gemini => value
                        .pointer("/candidates/0/finishReason")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|reason| !reason.is_empty()),
                };
                match (this.parse_event)(&event.data) {
                    Ok(chunk) => {
                        *this.terminal |= terminal;
                        *this.finished |= chunk
                            .as_ref()
                            .is_some_and(|chunk| chunk.choices.iter().any(|choice| choice.finish_reason.is_some()));
                        if let Some(chunk) = chunk {
                            return Poll::Ready(Some(Ok(chunk)));
                        }
                    }
                    Err(error) => {
                        *this.done = true;
                        return Poll::Ready(Some(Err(error)));
                    }
                }
            }
            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    if let Err(error) = this.decoder.push(&bytes) {
                        *this.done = true;
                        return Poll::Ready(Some(Err(error)));
                    }
                }
                Poll::Ready(Some(Err(error))) => {
                    *this.done = true;
                    return Poll::Ready(Some(Err(error.into())));
                }
                Poll::Ready(None) => {
                    *this.done = true;
                    if let Err(error) = this.decoder.finish() {
                        return Poll::Ready(Some(Err(error)));
                    }
                    let complete =
                        *this.terminal || (matches!(this.protocol, ChatStreamProtocol::OpenAi) && *this.finished);
                    return if complete {
                        Poll::Ready(None)
                    } else {
                        Poll::Ready(Some(Err(LiterLlmError::Streaming {
                            message: format!(
                                "{} stream ended before its final event; the response is incomplete \
                                 (no selected protocol terminal evidence)",
                                this.provider_name
                            ),
                        })))
                    };
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
