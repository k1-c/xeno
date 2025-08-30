use async_trait::async_trait;
use std::collections::HashMap;
use xeno_adapter_hyper::HyperAdapter;
use xeno_core::{App, CoreRequest, CoreResponse, Ctx, Error, Handler, IntoResponse};

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

struct UserHandler;

#[async_trait]
impl Handler<Ctx> for UserHandler {
    async fn call(&self, _ctx: Ctx, req: CoreRequest) -> Result<CoreResponse, Error> {
        let params = req
            .extensions()
            .get::<HashMap<String, String>>()
            .ok_or_else(|| Error::bad_request("Missing path parameters"))?;
        
        let user_id = params
            .get("id")
            .ok_or_else(|| Error::bad_request("Missing user ID"))?;
        
        let response_body = format!(r#"{{"user_id": "{}", "name": "User {}", "status": "active"}}"#, user_id, user_id);
        
        Ok(http::Response::builder()
            .status(200)
            .header("content-type", "application/json; charset=utf-8")
            .body(response_body.into())
            .unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ctx = Ctx::new();

    let app = App::new(ctx)
        .get("/", HelloHandler)
        .get("/health", HealthHandler)
        .get("/users/:id", UserHandler);

    let adapter = HyperAdapter::new(app);

    println!("Starting server on http://localhost:8080");
    adapter.serve("127.0.0.1:8080").await?;

    Ok(())
}
