use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

#[async_trait]
pub trait Kv: Send + Sync {
    async fn get(&self, key: &str) -> Option<Bytes>;
    async fn put(
        &self,
        key: &str,
        value: Bytes,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Clone)]
pub struct Ctx {
    pub kv: Option<Arc<dyn Kv>>,
}

impl Ctx {
    pub fn new() -> Self {
        Self { kv: None }
    }

    pub fn with_kv(kv: Arc<dyn Kv>) -> Self {
        Self { kv: Some(kv) }
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}
