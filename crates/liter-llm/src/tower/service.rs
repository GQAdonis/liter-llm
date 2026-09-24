use std::sync::Arc;
use std::task::{Context, Poll};

use tower::Service;

use super::types::{LlmRequest, LlmRequestKind, LlmResponse};
use crate::client::{BoxFuture, LlmClient};
use crate::error::{LiterLlmError, Result};

/// A thin tower [`Service`] wrapper around any [`LlmClient`] implementation.
///
/// Because [`LlmClient`] methods take `&self`, the inner client is stored
/// behind an [`Arc`] so the service can be cloned without owning a unique
/// reference.  `tower::Service::call` takes `&mut self`, but the actual
/// async work is dispatched through the shared reference inside the arc.
///
/// # Streaming behaviour
///
/// The client returns an owned `'static` stream, so the service forwards it
/// directly. Downstream polling supplies backpressure, dropping the response
/// closes the upstream stream, and cancelling `call` drops initialization.
#[cfg_attr(alef, alef(skip))]
pub struct LlmService<C> {
    inner: Arc<C>,
}

impl<C> LlmService<C> {
    /// Wrap `client` in a tower-compatible service.
    #[must_use]
    pub fn new(client: C) -> Self {
        Self {
            inner: Arc::new(client),
        }
    }

    /// Wrap a client that is already behind an `Arc`.
    ///
    /// This avoids a redundant `Arc` layer when the caller (e.g.
    /// [`ManagedClient`](crate::client::managed::ManagedClient)) already
    /// owns an `Arc<C>`.
    #[must_use]
    pub fn new_from_arc(client: Arc<C>) -> Self {
        Self { inner: client }
    }

    /// Return a reference to the inner client.
    pub fn inner(&self) -> &C {
        &self.inner
    }
}

impl<C> Clone for LlmService<C> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<C> Service<LlmRequest> for LlmService<C>
where
    C: LlmClient + Send + Sync + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, Result<LlmResponse>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let client = Arc::clone(&self.inner);
        Box::pin(async move {
            match req.kind {
                LlmRequestKind::Chat(r) => {
                    let resp = client.chat(r).await?;
                    Ok(LlmResponse::Chat(resp))
                }
                LlmRequestKind::ChatStream(r) => {
                    let stream = client.chat_stream(r).await?;
                    Ok(LlmResponse::ChatStream(stream))
                }
                LlmRequestKind::Embed(r) => {
                    let resp = client.embed(r).await?;
                    Ok(LlmResponse::Embed(resp))
                }
                LlmRequestKind::ListModels => {
                    let resp = client.list_models().await?;
                    Ok(LlmResponse::ListModels(resp))
                }
                LlmRequestKind::ImageGenerate(r) => {
                    let resp = client.image_generate(r).await?;
                    Ok(LlmResponse::ImageGenerate(resp))
                }
                LlmRequestKind::Speech(r) => {
                    let resp = client.speech(r).await?;
                    Ok(LlmResponse::Speech(resp))
                }
                LlmRequestKind::Transcribe(r) => {
                    let resp = client.transcribe(r).await?;
                    Ok(LlmResponse::Transcribe(resp))
                }
                LlmRequestKind::Moderate(r) => {
                    let resp = client.moderate(r).await?;
                    Ok(LlmResponse::Moderate(resp))
                }
                LlmRequestKind::Rerank(r) => {
                    let resp = client.rerank(r).await?;
                    Ok(LlmResponse::Rerank(resp))
                }
                LlmRequestKind::Search(r) => {
                    let resp = client.search(r).await?;
                    Ok(LlmResponse::Search(resp))
                }
                LlmRequestKind::Ocr(r) => {
                    let resp = client.ocr(r).await?;
                    Ok(LlmResponse::Ocr(resp))
                }
            }
        })
    }
}
