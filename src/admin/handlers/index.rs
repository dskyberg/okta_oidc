use std::sync::Arc;

use actix_web::{web::Data, Result};
use tracing::*;

use crate::{admin::pages::Index, AppState};

pub async fn index(app_state: Data<Arc<AppState>>) -> Result<Index> {
    let scopes = app_state.scopes();
    let claims = app_state.claims();
    info!("Scopes: {:#?}", &scopes);
    info!("Claims: {:#?}", &claims);
    Ok(Index { scopes, claims })
}
