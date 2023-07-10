use std::future::Ready;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

//pub struct Authentication;
//
//impl<A, B> Transform<A, ServiceRequest> for Authentication
//where
//    A: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//    A::Future: 'static,
//    B: 'static,
//{
//    type Response = ServiceResponse<B>;
//    type Error = Error;
//    type Transform = AuthMiddleware<A>;
//    type InitError = ();
//    type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//    fn new_transform(&self, service: A) -> Self::Future {
//        todo!()
//    }
//}
//
//pub struct AuthMiddleware<B> {
//    service: B,
//}

//impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
//where
//    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//    S::Future: 'static,
//    B: 'static,
//{
//    type Response = ServiceResponse<B>;
//    type Error = Error;
//    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//    fn poll_ready(
//        &self,
//        ctx: &mut core::task::Context<'_>,
//    ) -> std::task::Poll<Result<(), Self::Error>> {
//        todo!()
//    }
//
//    fn call(&self, req: ServiceRequest) -> Self::Future {
//        todo!()
//    }
//}
