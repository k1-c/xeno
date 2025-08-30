use crate::{CoreRequest, CoreResponse, Error, Handler};
use http::Method;
use matchit::{Match, Router as MatchItRouter};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router<C> {
    get_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    post_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    put_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    delete_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    patch_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    head_routes: MatchItRouter<Arc<dyn Handler<C>>>,
    options_routes: MatchItRouter<Arc<dyn Handler<C>>>,
}

impl<C: Send + Sync + Clone + 'static> Router<C> {
    pub fn new() -> Self {
        Self {
            get_routes: MatchItRouter::new(),
            post_routes: MatchItRouter::new(),
            put_routes: MatchItRouter::new(),
            delete_routes: MatchItRouter::new(),
            patch_routes: MatchItRouter::new(),
            head_routes: MatchItRouter::new(),
            options_routes: MatchItRouter::new(),
        }
    }

    pub fn add_route(&mut self, method: Method, path: &str, handler: Box<dyn Handler<C>>) {
        let handler_arc = Arc::from(handler);
        let result = match method {
            Method::GET => self.get_routes.insert(path, handler_arc),
            Method::POST => self.post_routes.insert(path, handler_arc),
            Method::PUT => self.put_routes.insert(path, handler_arc),
            Method::DELETE => self.delete_routes.insert(path, handler_arc),
            Method::PATCH => self.patch_routes.insert(path, handler_arc),
            Method::HEAD => self.head_routes.insert(path, handler_arc),
            Method::OPTIONS => self.options_routes.insert(path, handler_arc),
            _ => {
                eprintln!("Unsupported HTTP method: {}", method);
                return;
            }
        };

        if let Err(e) = result {
            eprintln!("Failed to insert route {} {}: {}", method, path, e);
        }
    }

    pub async fn handle(&self, ctx: C, mut req: CoreRequest) -> CoreResponse {
        let method = req.method().clone();
        let path = req.uri().path();

        let match_result = match method {
            Method::GET => self.get_routes.at(path),
            Method::POST => self.post_routes.at(path),
            Method::PUT => self.put_routes.at(path),
            Method::DELETE => self.delete_routes.at(path),
            Method::PATCH => self.patch_routes.at(path),
            Method::HEAD => self.head_routes.at(path),
            Method::OPTIONS => self.options_routes.at(path),
            _ => return self.method_not_allowed_response(),
        };

        match match_result {
            Ok(Match { value: handler, params }) => {
                let params_map: HashMap<String, String> = params
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect();
                req.extensions_mut().insert(params_map);

                match handler.call(ctx, req).await {
                    Ok(response) => response,
                    Err(error) => self.error_to_response(error),
                }
            }
            Err(_) => self.not_found_response(),
        }
    }

    fn error_to_response(&self, error: Error) -> CoreResponse {
        let status = error.status_code();
        
        #[cfg(debug_assertions)]
        let message = error.debug_message();
        
        #[cfg(not(debug_assertions))]
        let message = error.safe_message().to_string();
        
        let body = serde_json::json!({
            "error": message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        http::Response::builder()
            .status(status)
            .header("content-type", "application/json; charset=utf-8")
            .header("x-request-id", uuid::Uuid::new_v4().to_string())
            .body(body.to_string().into())
            .unwrap()
    }

    fn not_found_response(&self) -> CoreResponse {
        http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .header("content-type", "application/json; charset=utf-8")
            .body(r#"{"error":"Not Found"}"#.into())
            .unwrap()
    }

    fn method_not_allowed_response(&self) -> CoreResponse {
        http::Response::builder()
            .status(http::StatusCode::METHOD_NOT_ALLOWED)
            .header("content-type", "application/json; charset=utf-8")
            .body(r#"{"error":"Method Not Allowed"}"#.into())
            .unwrap()
    }
}

impl<C> Clone for Router<C> {
    fn clone(&self) -> Self {
        Self {
            get_routes: self.get_routes.clone(),
            post_routes: self.post_routes.clone(),
            put_routes: self.put_routes.clone(),
            delete_routes: self.delete_routes.clone(),
            patch_routes: self.patch_routes.clone(),
            head_routes: self.head_routes.clone(),
            options_routes: self.options_routes.clone(),
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
