use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use tokio::net::TcpListener;
use xeno_core::{App, CoreRequest, CoreResponse};

pub struct HyperAdapter<C> {
    app: App<C>,
}

impl<C: Send + Sync + Clone + 'static> HyperAdapter<C> {
    pub fn new(app: App<C>) -> Self {
        Self { app }
    }

    pub async fn serve(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server running on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let app = self.app.clone();
            let service = HyperService { app };

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
    ) -> Result<CoreRequest, Box<dyn std::error::Error + Send + Sync>> {
        let (parts, body) = req.into_parts();
        let body_bytes = body
            .collect()
            .await
            .map(|buf| buf.to_bytes())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let core_req = CoreRequest::from_parts(parts, Bytes::from(body_bytes));
        Ok(core_req)
    }

    fn convert_response(res: CoreResponse) -> Response<String> {
        let (parts, body) = res.into_parts();
        let body_str = String::from_utf8_lossy(&body).to_string();
        Response::from_parts(parts, body_str)
    }
}

impl<C: Send + Sync + Clone + 'static> Clone for HyperAdapter<C> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
        }
    }
}

struct HyperService<C> {
    app: App<C>,
}

impl<C: Send + Sync + Clone + 'static> Service<Request<Incoming>> for HyperService<C> {
    type Response = Response<String>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let app = self.app.clone();
        Box::pin(async move {
            let core_req = match HyperAdapter::<C>::convert_request(req).await {
                Ok(req) => req,
                Err(_) => {
                    let error_response = http::Response::builder()
                        .status(400)
                        .body(Bytes::from("Bad Request"))
                        .unwrap();
                    return Ok(HyperAdapter::<C>::convert_response(error_response));
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
        }
    }
}
