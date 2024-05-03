use crate::admin::pages::Index;

pub async fn index() -> actix_web::Result<Index> {
    Ok(Index {})
}
