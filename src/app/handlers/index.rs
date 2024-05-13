use actix_session::Session;
use actix_web::Result;
use serde_json::Value;
use std::collections::HashMap;
use tracing::*;

use crate::app::pages::Index;

#[instrument(skip(session))]
pub async fn index(session: Session) -> Result<Index> {
    // Grab session info
    let authenticated = session.get::<bool>("authenticated")?.unwrap_or(false);
    let id = session.get::<String>("username")?.unwrap_or_default();
    let userinfo = session.get::<String>("userinfo")?.unwrap_or_default();

    // Create template data
    let url = if authenticated { "/logout" } else { "/login" };
    let label = if authenticated { "Log Out" } else { "Log In" };
    let claims = get_userinfo_claims(&userinfo);

    info!("Authentication status: {}", &authenticated);

    Ok(Index {
        id,
        url: url.to_owned(),
        label: label.to_owned(),
        claims,
    })
}

/// Helper function to turn UserInfo json stuff into a key,value collection
fn get_userinfo_claims(userinfo: &str) -> HashMap<String, String> {
    let mut claims: HashMap<String, String> = HashMap::new();

    if !userinfo.is_empty() {
        let userinfo: Value =
            serde_json::from_str(userinfo).expect("Failed to parse claims from userinfo");
        let userinfo = userinfo
            .as_object()
            .expect("Failed to create object map from userinfo");
        for (key, value) in userinfo {
            claims.insert(key.to_owned(), serde_json::to_string(&value).unwrap());
        }
    }
    claims
}
