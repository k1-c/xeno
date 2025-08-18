use crate::{middleware::MiddlewareStack, router::Router, CoreRequest, CoreResponse, Ctx, Handler};
use http::Method;
use std::sync::Arc;

pub struct App<C = Ctx> {
    router: Arc<Router<C>>,
    middleware: Arc<MiddlewareStack<C>>,
    context: C,
}

impl<C: Send + Sync + Clone + 'static> App<C> {
    pub fn new(context: C) -> Self {
        Self {
            router: Arc::new(Router::new()),
            middleware: Arc::new(MiddlewareStack::new()),
            context,
        }
    }

    pub fn get(self, path: &str, handler: impl Handler<C> + 'static) -> Self {
        let mut router = Arc::try_unwrap(self.router).unwrap_or_else(|arc| (*arc).clone());
        router.add_route(Method::GET, path, Box::new(handler));

        Self {
            router: Arc::new(router),
            middleware: self.middleware,
            context: self.context,
        }
    }

    pub fn post(self, path: &str, handler: impl Handler<C> + 'static) -> Self {
        let mut router = Arc::try_unwrap(self.router).unwrap_or_else(|arc| (*arc).clone());
        router.add_route(Method::POST, path, Box::new(handler));

        Self {
            router: Arc::new(router),
            middleware: self.middleware,
            context: self.context,
        }
    }

    pub fn put(self, path: &str, handler: impl Handler<C> + 'static) -> Self {
        let mut router = Arc::try_unwrap(self.router).unwrap_or_else(|arc| (*arc).clone());
        router.add_route(Method::PUT, path, Box::new(handler));

        Self {
            router: Arc::new(router),
            middleware: self.middleware,
            context: self.context,
        }
    }

    pub fn delete(self, path: &str, handler: impl Handler<C> + 'static) -> Self {
        let mut router = Arc::try_unwrap(self.router).unwrap_or_else(|arc| (*arc).clone());
        router.add_route(Method::DELETE, path, Box::new(handler));

        Self {
            router: Arc::new(router),
            middleware: self.middleware,
            context: self.context,
        }
    }

    pub async fn handle(&self, req: CoreRequest) -> CoreResponse {
        self.router.handle(self.context.clone(), req).await
    }
}

impl<C: Clone> Clone for App<C> {
    fn clone(&self) -> Self {
        Self {
            router: Arc::clone(&self.router),
            middleware: Arc::clone(&self.middleware),
            context: self.context.clone(),
        }
    }
}

impl App<Ctx> {
    pub fn with_default_context() -> Self {
        Self::new(Ctx::default())
    }
}
