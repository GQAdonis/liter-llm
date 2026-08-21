//! Embedding-similarity vector store abstraction.
//!
//! [`VectorStore`] is the trait for K-nearest-neighbour lookup used by the
//! semantic cache tier.  Callers embed a prompt with an [`EmbeddingProvider`]
//! and then query the store to find a previously cached response whose
//! prompt is sufficiently similar to the current one.
//!
//! # Built-in implementations
//!
//! | Type | Description |
//! |---|---|
//! | [`InMemoryVectorStore`] | Brute-force cosine similarity over a `DashMap`. Suitable for ≤10 k entries. |
//! | [`OpenDalVectorStore`] | Persists vectors as JSON entries via any OpenDAL backend (gated on `opendal-cache`). |

pub mod memory;
#[cfg(feature = "opendal-cache")]
pub mod opendal;

pub use memory::InMemoryVectorStore;
#[cfg(feature = "opendal-cache")]
pub use opendal::OpenDalVectorStore;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::SystemTime;

use crate::error::Result;
use crate::types::{ContentPart, ImageUrl};

/// Metadata stored alongside each vector entry.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone)]
pub struct VectorMetadata {
    /// The exact-cache key this vector corresponds to.
    ///
    /// When a semantic match is found, the cache layer uses this key to look up
    /// the cached response in the exact-cache [`CacheStore`][crate::tower::cache::CacheStore].
    pub cache_key: u64,
    /// The serialized request body that was used when the entry was originally
    /// inserted into the exact-cache store.
    ///
    /// The semantic tier passes this to `CacheStore::get` instead of the
    /// current request's body so that the collision-guard check succeeds.
    /// Without this field the collision guard always fails for semantic hits
    /// because the current request body differs from the stored one by
    /// definition (they are only semantically similar, not byte-identical).
    pub original_request_body: String,
    /// Image payload associated with this vector, when the embedded item is an image.
    pub image_url: Option<ImageUrl>,
    /// Optional tenant identifier (for multi-tenant deployments).
    pub tenant_id: Option<String>,
    /// Wall-clock time when this vector was inserted.
    pub inserted_at: SystemTime,
    /// Arbitrary key-value metadata (model name, prompt hash, etc.).
    pub extra: HashMap<String, String>,
}

impl VectorMetadata {
    /// Convert the stored image payload into a chat content part.
    #[must_use]
    pub fn image_content_part(&self) -> Option<ContentPart> {
        self.image_url
            .clone()
            .map(|image_url| ContentPart::ImageUrl { image_url })
    }
}

/// Return `true` if an entry carrying `entry_tenant` is visible to a query
/// scoped to `query_tenant`.
///
/// ~keep The rule is plain equality, including the `None` case: a tenant-less
/// ~keep query only matches tenant-less entries, and a tenant-scoped query only
/// ~keep matches that exact tenant. "`None` matches everything" was
/// ~keep deliberately rejected — it would re-open the cross-tenant leak this
/// ~keep filter exists to close, since any request that happened to omit a
/// ~keep tenant would then see every other tenant's entries.
#[must_use]
pub(crate) fn tenant_matches(entry_tenant: Option<&str>, query_tenant: Option<&str>) -> bool {
    entry_tenant == query_tenant
}

/// A single result returned by [`VectorStore::search`].
#[derive(Debug, Clone)]
pub struct VectorMatch {
    /// Unique identifier of the matched vector.
    pub id: String,
    /// Cosine similarity score in the range `[−1.0, 1.0]`.
    pub similarity: f32,
    /// Metadata associated with the matched vector.
    pub metadata: VectorMetadata,
}

/// Pluggable vector store for the semantic cache tier.
///
/// All methods return pinned boxed futures so the trait is object-safe and can
/// be stored behind `Arc<dyn VectorStore>`.
///
/// # Implementing `VectorStore`
///
/// ```rust,ignore
/// use liter_llm::vectorstore::{VectorStore, VectorMatch, VectorMetadata};
/// use liter_llm::error::Result;
/// use std::future::Future;
/// use std::pin::Pin;
///
/// struct MyVectorStore;
///
/// impl VectorStore for MyVectorStore {
///     fn search<'a>(
///         &'a self,
///         query_vec: &'a [f32],
///         k: usize,
///         threshold: f32,
///         tenant_id: Option<&'a str>,
///     ) -> Pin<Box<dyn Future<Output = Vec<VectorMatch>> + Send + 'a>> {
///         Box::pin(async move { Vec::new() })
///     }
///
///     fn upsert<'a>(
///         &'a self,
///         id: String,
///         vec: Vec<f32>,
///         metadata: VectorMetadata,
///     ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
///         Box::pin(async move { Ok(()) })
///     }
///
///     fn delete<'a>(
///         &'a self,
///         id: &'a str,
///     ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
///         Box::pin(async move { Ok(()) })
///     }
///
///     fn dim(&self) -> usize { 1536 }
/// }
/// ```
pub trait VectorStore: Send + Sync + 'static {
    /// Find the K nearest neighbors above a similarity threshold, scoped to `tenant_id`.
    ///
    /// Returns at most `k` results sorted by descending similarity.  Only
    /// entries with `similarity >= threshold` **and** a matching tenant (per
    /// [`tenant_matches`]) are included. Pass `None` to search entries with no
    /// tenant set — `None` never matches a tenant-scoped entry, and a
    /// tenant-scoped query never matches a `None`-tenant entry.
    fn search<'a>(
        &'a self,
        query_vec: &'a [f32],
        k: usize,
        threshold: f32,
        tenant_id: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Vec<VectorMatch>> + Send + 'a>>;

    /// Insert or update a vector with associated metadata.
    ///
    /// If an entry with `id` already exists it is replaced.
    fn upsert<'a>(
        &'a self,
        id: String,
        vec: Vec<f32>,
        metadata: VectorMetadata,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

    /// Remove a vector by id.
    ///
    /// No-ops if the id does not exist.
    fn delete<'a>(&'a self, id: &'a str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

    /// Vector dimensionality the store expects.
    ///
    /// Callers should verify that the embedding dimension matches this value
    /// before calling [`upsert`][VectorStore::upsert].
    fn dim(&self) -> usize;
}
