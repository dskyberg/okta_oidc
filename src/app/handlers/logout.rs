use actix_session::Session;
use actix_web::{http::StatusCode, web, Responder};
use tracing::*;

#[instrument(skip(session))]
pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    info!("Session cleared");
    info!("Redirecting user to /");
    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
