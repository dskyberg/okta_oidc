//!
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{error::ErrorInternalServerError, web, Responder};
use log::info;
use openidconnect::{AuthorizationCode, CsrfToken, Nonce};

use crate::app_config::AppState;
use crate::auth::{AuthRequest, exchange_auth_code};


/// The user is redirected to the Authorization Server by the `/login` endpoint.
/// The Authorization Server authenticates the user, and returns a auth code.
/// This endpoint exchanges the auth code for a token response that contains the access_token and id_token.
pub async fn auth(
    session: Session,
    app_state: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> actix_web::Result<impl Responder> {

    info!("Recieved auth_code on front channel: {:?}", &params);

    // Get the nonce and state from the session, to validate the request
    let nonce = session.get::<Nonce>("oauth_nonce")?.unwrap();
    session.remove("oauth_nonce");

    // The state value provided in the request is returned in the response
    let _state = CsrfToken::new(params.state.clone());

    // The Authorization Code will be exchanged for the token
    let code = AuthorizationCode::new(params.code.clone());

    // Exchange the auth code for the token.  This does a back channel call to the Auth Server
    let userinfo = exchange_auth_code(code, _state, nonce, &app_state).await
    .map_err(ErrorInternalServerError)?;

    // For convenience, find an appropriate value to use as the display name
    let display_name = userinfo
        .get("name")
        .or(userinfo.get("preferred_username"))
        .or(userinfo.get("upn"))
        .or(userinfo.get("email"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Stuff the session and return the user to the home page.
    session.insert("display_name", display_name)?;
    session.insert("userinfo", serde_json::to_string_pretty(&userinfo)?)?;

    Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
}
