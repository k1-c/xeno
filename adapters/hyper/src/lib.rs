use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use tokio::net::TcpListener;
use xeno_core::{App, CoreRequest, CoreResponse, Error};

const DEFAULT_MAX_BODY_SIZE: usize = 2 * 1024 * 1024; // 2MB

pub struct HyperAdapter<C> {
    app: App<C>,
    max_body_size: usize,
}

impl<C: Send + Sync + Clone + 'static> HyperAdapter<C> {
    pub fn new(app: App<C>) -> Self {
        Self {
            app,
            max_body_size: DEFAULT_MAX_BODY_SIZE,
        }
    }

    pub fn with_max_body_size(mut self, max_size: usize) -> Self {
        self.max_body_size = max_size;
        self
    }

    pub async fn serve(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server running on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let app = self.app.clone();
            let max_body_size = self.max_body_size;
            let service = HyperService { app, max_body_size };

            tokio::spawn(async move {
                if let Err(err) = hyper::server::conn::http1::Builder::new()
                    .serve_connection(TokioIo::new(stream), service)
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    async fn convert_request(
        req: Request<Incoming>,
        max_body_size: usize,
    ) -> Result<CoreRequest, Error> {
        let (parts, body) = req.into_parts();

        let content_length = parts
            .headers
            .get("content-length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok());

        if let Some(length) = content_length {
            if length > max_body_size {
                return Err(Error::payload_too_large());
            }
        }

        let body_bytes = body
            .collect()
            .await
            .map(|buf| buf.to_bytes())
            .map_err(|_| Error::bad_request("Failed to read request body"))?;

        if body_bytes.len() > max_body_size {
            return Err(Error::payload_too_large());
        }

        let core_req = CoreRequest::from_parts(parts, body_bytes);
        Ok(core_req)
    }

    fn convert_response(res: CoreResponse) -> Response<String> {
        let (parts, body) = res.into_parts();
        let body_str = String::from_utf8_lossy(&body).to_string();
        Response::from_parts(parts, body_str)
    }

    fn error_to_response(error: Error) -> Response<String> {
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

        Response::builder()
            .status(status)
            .header("content-type", "application/json; charset=utf-8")
            .header("x-request-id", uuid::Uuid::new_v4().to_string())
            .body(body.to_string())
            .unwrap()
    }
}

impl<C: Send + Sync + Clone + 'static> Clone for HyperAdapter<C> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            max_body_size: self.max_body_size,
        }
    }
}

struct HyperService<C> {
    app: App<C>,
    max_body_size: usize,
}

impl<C: Send + Sync + Clone + 'static> Service<Request<Incoming>> for HyperService<C> {
    type Response = Response<String>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let app = self.app.clone();
        let max_body_size = self.max_body_size;
        Box::pin(async move {
            let core_req = match HyperAdapter::<C>::convert_request(req, max_body_size).await {
                Ok(req) => req,
                Err(error) => {
                    return Ok(HyperAdapter::<C>::error_to_response(error));
                }
            };

            let core_res = app.handle(core_req).await;
            Ok(HyperAdapter::<C>::convert_response(core_res))
        })
    }
}

impl<C: Send + Sync + Clone + 'static> Clone for HyperService<C> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            max_body_size: self.max_body_size,
        }
    }
}
