use crate::pages::IndexPage;
use actix_session::Session;

pub async fn index(session: Session) -> actix_web::Result<IndexPage> {
    let id = session
        .get::<String>("username")?
        .unwrap_or("anonymous".to_owned());
    let url = if id == "anonymous" {
        "/login"
    } else {
        "/logout"
    };
    let label = if id == "anonymous" {
        "Log In"
    } else {
        "Log Out"
    };
    let userinfo = session.get::<String>("userinfo")?.unwrap_or_default();

    Ok(IndexPage {
        id,
        url: url.to_owned(),
        label: label.to_owned(),
        userinfo,
    })
}
