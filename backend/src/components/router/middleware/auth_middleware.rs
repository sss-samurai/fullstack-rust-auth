use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpMessage,
    dev::{ServiceRequest, ServiceResponse},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::components::utils::user_authentication::{
    decrypt_encrypted_token::decrypt_encrypted_token, get_token::get_token,
};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self,  req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let secret = std::env::var("KEY").expect("KEY must be set");

            match get_token(req.request()) {
                Ok(token) => match decrypt_encrypted_token(&token, &secret) {
                    Ok(claims) => {
                        println!("Decrypted claims: {:?}", claims);
                        req.extensions_mut().insert(claims);
                        service.call(req).await
                    }
                    Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
                },
                Err(_) => Err(actix_web::error::ErrorUnauthorized("Missing token")),
            }
        })
    }
}
