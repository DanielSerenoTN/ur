use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::auth::auth_service_trait::AuthServiceTrait;

#[derive(Clone)]
pub struct AuthGuard {
    auth_service: Arc<dyn AuthServiceTrait>,
}

impl AuthGuard {
    pub fn new(auth_service: Arc<dyn AuthServiceTrait>) -> Self {
        AuthGuard { auth_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthGuardImpl {
            service: Arc::new(service),
            auth_service: Arc::clone(&self.auth_service),
        })
    }
}

pub struct AuthGuardImpl<S> {
    service: Arc<S>,
    auth_service: Arc<dyn AuthServiceTrait>,
}

impl<S, B> Service<ServiceRequest> for AuthGuardImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let service = Arc::clone(&self.service);
        let auth_service = Arc::clone(&self.auth_service);
        let token_opt = auth_service.extract_token(&req);
        Box::pin(async move {
            if let Some(token) = token_opt {
                match auth_service.verify_token(&token){
                    Ok(claims) => {
                        req.extensions_mut().insert(claims);
                        let res = service.call(req).await?;
                        Ok(res)
                    }
                    Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid or expired token")),
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("Authorization token missing"))
            }
        })
    }
}
