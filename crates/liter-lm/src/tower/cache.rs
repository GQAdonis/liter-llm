//! Response caching middleware.
//!
//! [`CacheLayer`] wraps any [`Service<LlmRequest>`] and caches non-streaming
//! responses keyed by a hash of the serialised request.  Only
//! [`LlmResponse::Chat`] and [`LlmResponse::Embed`] responses are cached;
//! streaming, model-list, and other response variants are passed through
//! uncached.
//!
//! The cache is a simple in-memory LRU with a configurable maximum entry count
//! and TTL.  It uses [`std::collections::HashMap`] with a [`VecDeque`] for LRU
//! eviction order.

use std::collections::{HashMap, VecDeque};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use tower::{Layer, Service};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};
use crate::types::{ChatCompletionResponse, EmbeddingResponse};

// ---- Config ----------------------------------------------------------------

/// Configuration for the response cache.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of cached entries.
    pub max_entries: usize,
    /// Time-to-live for each cached entry.
    pub ttl: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 256,
            ttl: Duration::from_secs(300),
        }
    }
}

// ---- Cache entry -----------------------------------------------------------

/// A cached response with its insertion timestamp.
#[derive(Clone)]
struct CacheEntry {
    response: CachedResponse,
    inserted_at: Instant,
}

/// The subset of [`LlmResponse`] variants that can be cached.
///
/// Streaming responses are not cacheable because they are consumed once.
#[derive(Clone)]
enum CachedResponse {
    Chat(ChatCompletionResponse),
    Embed(EmbeddingResponse),
}

impl CachedResponse {
    fn into_llm_response(self) -> LlmResponse {
        match self {
            Self::Chat(r) => LlmResponse::Chat(r),
            Self::Embed(r) => LlmResponse::Embed(r),
        }
    }
}

// ---- Inner cache -----------------------------------------------------------

struct InnerCache {
    map: HashMap<u64, CacheEntry>,
    /// Keys in insertion order (front = oldest).
    order: VecDeque<u64>,
    max_entries: usize,
    ttl: Duration,
}

impl InnerCache {
    fn new(config: &CacheConfig) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            max_entries: config.max_entries,
            ttl: config.ttl,
        }
    }

    fn get(&mut self, key: u64) -> Option<CachedResponse> {
        let entry = self.map.get(&key)?;
        if entry.inserted_at.elapsed() > self.ttl {
            // Expired — remove from map (the order deque will be cleaned lazily).
            self.map.remove(&key);
            return None;
        }
        Some(entry.response.clone())
    }

    fn insert(&mut self, key: u64, response: CachedResponse) {
        // Evict oldest entries if at capacity.
        while self.map.len() >= self.max_entries {
            if let Some(oldest_key) = self.order.pop_front() {
                self.map.remove(&oldest_key);
            } else {
                break;
            }
        }

        self.map.insert(
            key,
            CacheEntry {
                response,
                inserted_at: Instant::now(),
            },
        );
        self.order.push_back(key);
    }
}

// ---- Layer -----------------------------------------------------------------

/// Tower [`Layer`] that caches non-streaming LLM responses.
pub struct CacheLayer {
    cache: Arc<RwLock<InnerCache>>,
}

impl CacheLayer {
    /// Create a new cache layer with the given configuration.
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(InnerCache::new(&config))),
        }
    }
}

impl<S> Layer<S> for CacheLayer {
    type Service = CacheService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheService {
            inner,
            cache: Arc::clone(&self.cache),
        }
    }
}

// ---- Service ---------------------------------------------------------------

/// Tower service produced by [`CacheLayer`].
pub struct CacheService<S> {
    inner: S,
    cache: Arc<RwLock<InnerCache>>,
}

impl<S: Clone> Clone for CacheService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            cache: Arc::clone(&self.cache),
        }
    }
}

/// Compute a cache key from the request.
///
/// Only `Chat` and `Embed` requests are cacheable.  Returns `None` for all
/// other request variants (streaming, `ListModels`, image, audio, etc.).
fn cache_key(req: &LlmRequest) -> Option<u64> {
    let json = match req {
        LlmRequest::Chat(r) => serde_json::to_string(r).ok()?,
        LlmRequest::Embed(r) => serde_json::to_string(r).ok()?,
        // Not cacheable.
        _ => return None,
    };

    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    Some(hasher.finish())
}

impl<S> Service<LlmRequest> for CacheService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let key = cache_key(&req);

        // Check cache on read path.
        if let Some(k) = key
            && let Ok(mut cache) = self.cache.write()
            && let Some(cached) = cache.get(k)
        {
            return Box::pin(async move { Ok(cached.into_llm_response()) });
        }

        let cache = Arc::clone(&self.cache);
        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;

            // Store cacheable responses.
            if let Some(k) = key {
                let cached = match &resp {
                    LlmResponse::Chat(r) => Some(CachedResponse::Chat(r.clone())),
                    LlmResponse::Embed(r) => Some(CachedResponse::Embed(r.clone())),
                    _ => None,
                };
                if let Some(cached) = cached
                    && let Ok(mut cache) = cache.write()
                {
                    cache.insert(k, cached);
                }
            }

            Ok(resp)
        })
    }
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use tower::{Layer as _, Service as _};

    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;

    #[tokio::test]
    async fn cache_returns_cached_response_on_second_call() {
        let config = CacheConfig {
            max_entries: 10,
            ttl: Duration::from_secs(60),
        };
        let layer = CacheLayer::new(config);
        let client = MockClient::ok();
        let call_count = Arc::clone(&client.call_count);
        let inner = LlmService::new(client);
        let mut svc = layer.layer(inner);

        // First call — cache miss.
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Second call — cache hit.
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
        assert_eq!(call_count.load(Ordering::SeqCst), 1, "second call should hit cache");
    }

    #[tokio::test]
    async fn cache_does_not_cache_streaming_requests() {
        let config = CacheConfig {
            max_entries: 10,
            ttl: Duration::from_secs(60),
        };
        let layer = CacheLayer::new(config);
        let client = MockClient::ok();
        let call_count = Arc::clone(&client.call_count);
        let inner = LlmService::new(client);
        let mut svc = layer.layer(inner);

        svc.call(LlmRequest::ChatStream(chat_req("gpt-4"))).await.unwrap();
        svc.call(LlmRequest::ChatStream(chat_req("gpt-4"))).await.unwrap();
        assert_eq!(call_count.load(Ordering::SeqCst), 2, "streaming should not be cached");
    }

    #[tokio::test]
    async fn cache_evicts_oldest_when_full() {
        let config = CacheConfig {
            max_entries: 1,
            ttl: Duration::from_secs(60),
        };
        let layer = CacheLayer::new(config);
        let client = MockClient::ok();
        let call_count = Arc::clone(&client.call_count);
        let inner = LlmService::new(client);
        let mut svc = layer.layer(inner);

        // Fill cache with model-a.
        svc.call(LlmRequest::Chat(chat_req("model-a"))).await.unwrap();
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Insert model-b, evicting model-a.
        svc.call(LlmRequest::Chat(chat_req("model-b"))).await.unwrap();
        assert_eq!(call_count.load(Ordering::SeqCst), 2);

        // model-a should be evicted — cache miss.
        svc.call(LlmRequest::Chat(chat_req("model-a"))).await.unwrap();
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            3,
            "evicted entry should be a cache miss"
        );
    }

    #[tokio::test]
    async fn cache_different_requests_have_different_keys() {
        let config = CacheConfig {
            max_entries: 10,
            ttl: Duration::from_secs(60),
        };
        let layer = CacheLayer::new(config);
        let client = MockClient::ok();
        let call_count = Arc::clone(&client.call_count);
        let inner = LlmService::new(client);
        let mut svc = layer.layer(inner);

        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
        svc.call(LlmRequest::Chat(chat_req("gpt-3.5-turbo"))).await.unwrap();
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            2,
            "different models should be cache misses"
        );
    }
}
