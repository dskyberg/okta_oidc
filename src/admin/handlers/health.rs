use actix_web::Responder;

pub async fn health() -> impl Responder {
    "All good\n"
}
