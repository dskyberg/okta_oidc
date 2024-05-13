use actix_web::{Responder, Result};

pub async fn health() -> Result<impl Responder> {
    Ok("All good\n")
}
