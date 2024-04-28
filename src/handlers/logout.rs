use actix_session::Session;
use actix_web::{http::StatusCode, web, Responder};

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
