use async_trait::async_trait;
use xeno_adapter_workers::{WorkerRequest, WorkerResponse, WorkersAdapter};
use xeno_core::{App, CoreRequest, CoreResponse, Ctx, Error, Handler, IntoResponse};

struct HelloHandler;

#[async_trait]
impl Handler<Ctx> for HelloHandler {
    async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse, Error> {
        Ok("Hello from Cloudflare Workers with Xeno!".into_response())
    }
}

struct HealthHandler;

#[async_trait]
impl Handler<Ctx> for HealthHandler {
    async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse, Error> {
        Ok("OK".into_response())
    }
}

// This will be the main entry point for Cloudflare Workers
pub async fn main(req: WorkerRequest) -> WorkerResponse {
    let ctx = Ctx::new();

    let app = App::new(ctx)
        .get("/", HelloHandler)
        .get("/health", HealthHandler);

    let adapter = WorkersAdapter::new(app);
    adapter.handle_fetch(req).await
}
