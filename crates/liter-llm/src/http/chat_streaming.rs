//! Chat-only terminal contract. Other SSE endpoints retain their own semantics.
use std::{
    pin::Pin,
    sync::Arc,
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
    provider::Provider,
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

pub(crate) async fn post_chat_stream(
    client: &reqwest::Client,
    request: StreamingPost<'_>,
    provider: Arc<dyn Provider>,
    options: ResponseReadOptions,
) -> Result<crate::client::BoxStream<'static, Result<ChatCompletionChunk>>> {
    let response = request.send(client, options).await?;
    let protocol = provider.chat_stream_protocol();
    Ok(Box::pin(ChatSseParser {
        inner: response.bytes_stream(),
        decoder: SseDecoder::default(),
        provider,
        protocol,
        terminal: false,
        done: false,
    }))
}

pin_project! {
    struct ChatSseParser<S> {
        #[pin]
        inner: S,
        decoder: SseDecoder,
        provider: Arc<dyn Provider>,
        protocol: ChatStreamProtocol,
        terminal: bool,
        done: bool,
    }
}

fn failure(message: &str) -> LiterLlmError {
    LiterLlmError::Streaming {
        message: message.into(),
    }
}

impl<S> Stream for ChatSseParser<S>
where
    S: Stream<Item = std::result::Result<Bytes, reqwest::Error>>,
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
                    return Poll::Ready(Some(Err(failure(
                        "Provider sent data after the terminal event",
                    ))));
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
                        return Poll::Ready(Some(Err(failure(
                            "Malformed provider SSE JSON event",
                        ))));
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
                match this.provider.parse_stream_event(&event.data) {
                    Ok(chunk) => {
                        *this.terminal |= terminal;
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
                    return if *this.terminal {
                        Poll::Ready(None)
                    } else {
                        Poll::Ready(Some(Err(failure(
                            "Provider stream ended without selected protocol terminal evidence",
                        ))))
                    };
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
