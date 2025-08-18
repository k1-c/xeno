use crate::{CoreRequest, CoreResponse, Error, Handler};
use async_trait::async_trait;

#[async_trait]
pub trait Middleware<C: Send + Sync + Clone + 'static>: Send + Sync {
    async fn before(&self, ctx: &C, req: &mut CoreRequest) -> Result<(), Error> {
        let _ = (ctx, req);
        Ok(())
    }

    async fn after(&self, ctx: &C, req: &CoreRequest, res: &mut CoreResponse) -> Result<(), Error> {
        let _ = (ctx, req, res);
        Ok(())
    }
}

pub struct MiddlewareStack<C> {
    middleware: Vec<Box<dyn Middleware<C>>>,
}

impl<C: Send + Sync + Clone + 'static> MiddlewareStack<C> {
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
        }
    }

    pub fn add(&mut self, middleware: Box<dyn Middleware<C>>) {
        self.middleware.push(middleware);
    }

    pub async fn execute<H>(&self, ctx: C, mut req: CoreRequest, handler: &H) -> CoreResponse
    where
        H: Handler<C>,
    {
        for middleware in &self.middleware {
            if let Err(error) = middleware.before(&ctx, &mut req).await {
                return self.error_to_response(error);
            }
        }

        let mut response = match handler.call(ctx.clone(), req.clone()).await {
            Ok(res) => res,
            Err(error) => return self.error_to_response(error),
        };

        for middleware in self.middleware.iter().rev() {
            if let Err(error) = middleware.after(&ctx, &req, &mut response).await {
                return self.error_to_response(error);
            }
        }

        response
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
}

impl<C> Default for MiddlewareStack<C>
where
    C: Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
