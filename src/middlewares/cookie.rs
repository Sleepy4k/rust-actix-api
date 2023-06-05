use std::{env, future::{ready, Ready}};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Validation, DecodingKey};
use actix_web::{Error, body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}};

use crate::{helpers::response::response_json, structs::auth::TokenStruct};

pub struct CheckCookie;

impl<S, B> Transform<S, ServiceRequest> for CheckCookie
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckCookieMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckCookieMiddleware { service }))
    }
}
pub struct CheckCookieMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckCookieMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let path = request.path().to_string();
        let token: Option<actix_web::cookie::Cookie> = request.cookie("auth_jwt_secret");

        if path == "/" || path == "/login" || path == "/register" {
            let res = self.service.call(request);

            return Box::pin(async move {
                res.await.map(ServiceResponse::map_into_left_body)
            });
        }

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"));
        let validation = Validation::default();

        match token {
            Some(token) => {
                match decode::<TokenStruct> (
                    &token.value(),
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &validation
                ) {
                    Ok(_) => {
                        let res = self.service.call(request);

                        return Box::pin(async move {
                            res.await.map(ServiceResponse::map_into_left_body)
                        });
                    },
                    Err(_) => {
                        let request = request.into_parts().0;

                        let response = response_json(
                            "unauthorize".to_string(),
                            "something went wrong from your cookies".to_string(),
                            vec![]
                        ).map_into_right_body();

                        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                    }
                }
            }
            None => {
                let request = request.into_parts().0;

                let response = response_json(
                    "unauthorize".to_string(),
                    "please authorize your self as user".to_string(),
                    vec![]
                ).map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        }
    }
}