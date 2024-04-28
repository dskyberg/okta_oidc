use actix_web::{http::StatusCode, web, Responder};

use crate::auth::OidcClient;

pub async fn login(oidc: web::Data<OidcClient>) -> impl Responder {
    web::Redirect::to(oidc.authorize_url.clone()).using_status_code(StatusCode::FOUND)
}
