use xeno_adapter_hyper::HyperAdapter;
use xeno_core::{App, Handler, CoreRequest, CoreResponse, Error, Ctx};
use async_trait::async_trait;

struct HelloHandler;

#[async_trait]
impl Handler<Ctx> for HelloHandler {
    async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse, Error> {
        Ok("Hello, World!".into_response())
    }
}

struct HealthHandler;

#[async_trait]
impl Handler<Ctx> for HealthHandler {
    async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse, Error> {
        Ok("OK".into_response())
    }
}

use xeno_core::IntoResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = Ctx::new();
    
    let app = App::new(ctx)
        .get("/", HelloHandler)
        .get("/health", HealthHandler);
    
    let adapter = HyperAdapter::new(app);
    
    println!("Starting server on http://localhost:8080");
    adapter.serve("127.0.0.1:8080").await?;
    
    Ok(())
}