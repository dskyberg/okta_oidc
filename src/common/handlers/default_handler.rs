use crate::common::pages::NotFoundPage;
use actix_web::{http::Method, Either, HttpResponse, Responder};

pub async fn default_handler(req_method: Method) -> actix_web::Result<impl Responder> {
    match req_method {
        Method::GET => Ok(Either::Left(NotFoundPage {})),
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}
