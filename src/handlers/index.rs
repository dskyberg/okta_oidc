use std::collections::HashMap;

use crate::pages::IndexPage;
use actix_session::Session;
use log::info;
use serde_json::Value;

pub async fn index(session: Session) -> actix_web::Result<IndexPage> {
    let id = session
        .get::<String>("username")?
        .unwrap_or_default();

    let url = if id == "" {
        "/login"
    } else {
        "/logout"
    };
    let label = if id == "" {
        "Log In"
    } else {
        "Log Out"
    };
    let userinfo = session.get::<String>("userinfo")?.unwrap_or_default();
    info!("UserInfo found in session: {}", &userinfo);
    let mut claims: HashMap<String,String> = HashMap::new();

    if !userinfo.is_empty() {
    let userinfo: Value = serde_json::from_str(&userinfo).expect("Failed to parse claims from userinfo");
    let userinfo = userinfo.as_object().expect("Failed to create object map from userinfo");
    for (key, value) in userinfo {
        claims.insert(key.to_owned(), serde_json::to_string(&value).unwrap());
    }
    }
    Ok(IndexPage {
        id,
        url: url.to_owned(),
        label: label.to_owned(),
        claims,
    })
}
