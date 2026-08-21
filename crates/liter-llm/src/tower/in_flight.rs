//! In-flight request concurrency limiting middleware.

use std::sync::Arc;
use std::task::{Context, Poll};

use tokio::sync::Semaphore;
use tower::{Layer, Service, ServiceExt};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};

/// Configuration for the global per-client in-flight request limit.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InFlightLimitConfig {
    /// Maximum simultaneously outstanding provider requests. `None` means unlimited.
    pub max_in_flight: Option<usize>,
}

/// Tower [`Layer`] that queues requests until a provider concurrency permit is available.
#[cfg_attr(alef, alef(skip))]
#[derive(Clone)]
pub struct InFlightLimitLayer {
    semaphore: Option<Arc<Semaphore>>,
}

impl InFlightLimitLayer {
    /// Create an in-flight limit layer.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::BadRequest`] when `max_in_flight` is zero.
    pub fn new(config: InFlightLimitConfig) -> Result<Self> {
        let semaphore = match config.max_in_flight {
            Some(0) => {
                return Err(LiterLlmError::BadRequest {
                    message: "max_in_flight must be greater than zero".into(),
                    status: 400,
                });
            }
            Some(max) => Some(Arc::new(Semaphore::new(max))),
            None => None,
        };
        Ok(Self { semaphore })
    }
}

impl<S> Layer<S> for InFlightLimitLayer {
    type Service = InFlightLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        InFlightLimitService {
            inner,
            semaphore: self.semaphore.clone(),
        }
    }
}

/// Tower service produced by [`InFlightLimitLayer`].
#[cfg_attr(alef, alef(skip))]
pub struct InFlightLimitService<S> {
    inner: S,
    semaphore: Option<Arc<Semaphore>>,
}

impl<S: Clone> Clone for InFlightLimitService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            semaphore: self.semaphore.clone(),
        }
    }
}

impl<S> Service<LlmRequest> for InFlightLimitService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, Result<LlmResponse>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let semaphore = self.semaphore.clone();
        let inner = self.inner.clone();

        Box::pin(async move {
            let _permit = match semaphore {
                Some(semaphore) => Some(
                    semaphore
                        .acquire_owned()
                        .await
                        .map_err(|_| LiterLlmError::InternalError {
                            message: "in-flight limiter closed unexpectedly".into(),
                        })?,
                ),
                None => None,
            };
            inner.oneshot(req).await
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tower::{Layer, ServiceExt, service_fn};

    use super::*;
    use crate::types::ChatCompletionRequest;

    #[tokio::test]
    async fn limits_observed_peak_in_flight_requests() {
        let current = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let inner = service_fn({
            let current = current.clone();
            let peak = peak.clone();
            move |_req: LlmRequest| {
                let current = current.clone();
                let peak = peak.clone();
                async move {
                    let active = current.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(active, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(25)).await;
                    current.fetch_sub(1, Ordering::SeqCst);
                    Err::<LlmResponse, LiterLlmError>(LiterLlmError::InternalError {
                        message: "test completion".into(),
                    })
                }
            }
        });
        let layer = InFlightLimitLayer::new(InFlightLimitConfig { max_in_flight: Some(2) }).unwrap();
        let service = layer.layer(inner);

        let calls = (0..6).map(|_| {
            service.clone().oneshot(LlmRequest::Chat(ChatCompletionRequest {
                model: "gpt-4".into(),
                ..Default::default()
            }))
        });
        let _ = futures_util::future::join_all(calls).await;

        assert_eq!(peak.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn rejects_zero_limit() {
        let result = InFlightLimitLayer::new(InFlightLimitConfig { max_in_flight: Some(0) });
        assert!(matches!(result, Err(LiterLlmError::BadRequest { status: 400, .. })));
    }
}
