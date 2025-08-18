use crate::{CoreRequest, CoreResponse, Error, Handler};
use http::Method;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router<C> {
    routes: HashMap<String, Arc<dyn Handler<C>>>,
}

impl<C: Send + Sync + Clone + 'static> Router<C> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Box<dyn Handler<C>>) {
        let route_key = format!("{} {}", method, path);
        self.routes.insert(route_key, handler.into());
    }

    pub async fn handle(&self, ctx: C, mut req: CoreRequest) -> CoreResponse {
        let method = req.method().clone();
        let path = req.uri().path();
        let route_key = format!("{} {}", method, path);

        match self.routes.get(&route_key) {
            Some(handler) => {
                let params: HashMap<String, String> = HashMap::new();
                req.extensions_mut().insert(params);

                match handler.call(ctx, req).await {
                    Ok(response) => response,
                    Err(error) => self.error_to_response(error),
                }
            }
            None => self.not_found_response(),
        }
    }

    fn error_to_response(&self, error: Error) -> CoreResponse {
        let status = error.status_code();
        let body = format!("{{\"error\":\"{}\"}}", error);

        http::Response::builder()
            .status(status)
            .header("content-type", "application/json; charset=utf-8")
            .body(body.into())
            .unwrap()
    }

    fn not_found_response(&self) -> CoreResponse {
        http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .header("content-type", "application/json; charset=utf-8")
            .body(r#"{"error":"Not Found"}"#.into())
            .unwrap()
    }
}

impl<C> Clone for Router<C> {
    fn clone(&self) -> Self {
        Self {
            routes: self.routes.clone(),
        }
    }
}

impl<C> Default for Router<C>
where
    C: Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
