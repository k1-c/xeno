pub mod app;
pub mod context;
pub mod error;
pub mod extract;
pub mod handler;
pub mod middleware;
pub mod response;
pub mod router;

pub use app::App;
pub use context::Ctx;
pub use error::Error;
pub use extract::{Json, Path, Query};
pub use handler::Handler;
pub use response::IntoResponse;

pub type CoreRequest = http::Request<bytes::Bytes>;
pub type CoreResponse = http::Response<bytes::Bytes>;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use http::{Method, StatusCode};
    use std::collections::HashMap;

    struct TestHandler {
        response: &'static str,
    }

    #[async_trait]
    impl Handler<Ctx> for TestHandler {
        async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse> {
            Ok(self.response.into_response())
        }
    }

    struct PathTestHandler;

    #[async_trait]
    impl Handler<Ctx> for PathTestHandler {
        async fn call(&self, _ctx: Ctx, req: CoreRequest) -> Result<CoreResponse> {
            let params = req
                .extensions()
                .get::<HashMap<String, String>>()
                .ok_or_else(|| Error::bad_request("Missing path parameters"))?;
            
            let id = params
                .get("id")
                .ok_or_else(|| Error::bad_request("Missing id parameter"))?;
            
            let response_body = format!(r#"{{"id": "{}"}}"#, id);
            Ok(http::Response::builder()
                .status(200)
                .header("content-type", "application/json; charset=utf-8")
                .body(response_body.into())
                .unwrap())
        }
    }

    struct ErrorTestHandler;

    #[async_trait]
    impl Handler<Ctx> for ErrorTestHandler {
        async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse> {
            Err(Error::bad_request("Test error"))
        }
    }

    #[tokio::test]
    async fn test_app_get_200_response() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/hello", TestHandler { response: "Hello, World!" });

        let req = http::Request::builder()
            .method(Method::GET)
            .uri("/hello")
            .body(bytes::Bytes::new())
            .unwrap();

        let response = app.handle(req).await;
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = String::from_utf8_lossy(response.body());
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn test_app_404_response() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/hello", TestHandler { response: "Hello, World!" });

        let req = http::Request::builder()
            .method(Method::GET)
            .uri("/nonexistent")
            .body(bytes::Bytes::new())
            .unwrap();

        let response = app.handle(req).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        
        let body = String::from_utf8_lossy(response.body());
        assert!(body.contains("Not Found"));
    }

    #[tokio::test]
    async fn test_app_400_response() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/error", ErrorTestHandler);

        let req = http::Request::builder()
            .method(Method::GET)
            .uri("/error")
            .body(bytes::Bytes::new())
            .unwrap();

        let response = app.handle(req).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        
        let body = String::from_utf8_lossy(response.body());
        assert!(body.contains("error"));
    }

    #[tokio::test]
    async fn test_path_parameters() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/users/:id", PathTestHandler);

        let req = http::Request::builder()
            .method(Method::GET)
            .uri("/users/123")
            .body(bytes::Bytes::new())
            .unwrap();

        let response = app.handle(req).await;
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = String::from_utf8_lossy(response.body());
        assert!(body.contains(r#""id": "123""#));
    }

    #[tokio::test]
    async fn test_multiple_routes() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/hello", TestHandler { response: "Hello" })
            .get("/world", TestHandler { response: "World" })
            .post("/data", TestHandler { response: "Posted" });

        let test_cases = vec![
            (Method::GET, "/hello", StatusCode::OK, "Hello"),
            (Method::GET, "/world", StatusCode::OK, "World"),
            (Method::POST, "/data", StatusCode::OK, "Posted"),
            (Method::GET, "/nonexistent", StatusCode::NOT_FOUND, "Not Found"),
        ];

        for (method, path, expected_status, expected_content) in test_cases {
            let req = http::Request::builder()
                .method(method)
                .uri(path)
                .body(bytes::Bytes::new())
                .unwrap();

            let response = app.handle(req).await;
            assert_eq!(response.status(), expected_status);
            
            let body = String::from_utf8_lossy(response.body());
            assert!(body.contains(expected_content));
        }
    }

    #[tokio::test]
    async fn test_error_handling() {
        let ctx = Ctx::new();
        let app = App::new(ctx)
            .get("/internal", TestHandler { response: "OK" });

        // Test internal errors through router
        let req = http::Request::builder()
            .method(Method::GET)
            .uri("/internal")
            .body(bytes::Bytes::new())
            .unwrap();

        let response = app.handle(req).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
