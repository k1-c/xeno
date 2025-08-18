use crate::{CoreRequest, CoreResponse, Error};
use async_trait::async_trait;

#[async_trait]
pub trait Handler<C: Send + Sync + Clone + 'static>: Send + Sync {
    async fn call(&self, ctx: C, req: CoreRequest) -> Result<CoreResponse, Error>;
}
