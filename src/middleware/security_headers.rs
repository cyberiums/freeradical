use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, http};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SecurityHeadersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(
                http::header::STRICT_TRANSPORT_SECURITY,
                http::header::HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
            );
            res.headers_mut().insert(
                http::header::X_XSS_PROTECTION,
                http::header::HeaderValue::from_static("1; mode=block"),
            );
            res.headers_mut().insert(
                http::header::X_CONTENT_TYPE_OPTIONS,
                http::header::HeaderValue::from_static("nosniff"),
            );
            res.headers_mut().insert(
                http::header::X_FRAME_OPTIONS,
                http::header::HeaderValue::from_static("SAMEORIGIN"),
            );
            // Basic CSP - permissive for now to avoid breaking inline scripts/styles in dashboard
            // In strict mode, we would require nonces/hashes.
            res.headers_mut().insert(
                http::header::CONTENT_SECURITY_POLICY,
                http::header::HeaderValue::from_static("default-src 'self' data: https: 'unsafe-inline' 'unsafe-eval'; img-src 'self' data: https:; connect-src 'self' http://localhost:8000 http://localhost:8080 https:;"),
            );
             res.headers_mut().insert(
                http::header::REFERRER_POLICY,
                http::header::HeaderValue::from_static("strict-origin-when-cross-origin"),
            );

            Ok(res)
        })
    }
}
