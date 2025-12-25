use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::rc::Rc;
use std::cell::RefCell;
use crate::services::plugin_service::PluginRegistry;
use std::sync::Arc;

pub struct PluginMiddleware {
    registry: Arc<PluginRegistry>,
}

impl PluginMiddleware {
    pub fn new(registry: Arc<PluginRegistry>) -> Self {
        Self { registry }
    }
}

impl<S, B> Transform<S> for PluginMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PluginMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PluginMiddlewareService {
            service: Rc::new(RefCell::new(service)),
            registry: self.registry.clone(),
        }))
    }
}

pub struct PluginMiddlewareService<S> {
    service: Rc<RefCell<S>>,
    registry: Arc<PluginRegistry>,
}

impl<S, B> Service for PluginMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let registry = self.registry.clone();

        Box::pin(async move {
            registry.execute_on_request(&req).await;
            
            let fut = service.borrow_mut().call(req);
            fut.await
        })
    }
}
