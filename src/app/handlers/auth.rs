use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::{error::ErrorInternalServerError, web, Responder};
use tracing::*;

//use tracing_attributes::instrument;
use openidconnect::{AuthorizationCode, CsrfToken, Nonce};

use crate::auth::{exchange_auth_code, AuthRequest};
use crate::AppState;

/// After successfull authentication, the Auth Server redirects the user to this url,
/// as determined by the `redirect_url` parameter passed on the redirect from /login to the Auth Server's
/// `/authorize` endpoint.
///  if the `response_type=code`,then the Authorization Server authenticates the user, and redirect to this endpoint.
/// The redirect will include the resulting auth code and state that were included on the
/// This endpoint exchanges the auth code for a token response that contains the access_token and id_token.
#[instrument(skip(session, app_state, req))]
pub async fn auth(
    session: Session,
    app_state: web::Data<AppState>,
    params: web::Query<AuthRequest>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    if let Some(referrer) = req.headers().get("referrer") {
        info!("auth referrer: {:?}", referrer);
    } else {
        info!("No referrer to auth");
    }

    info!("Recieved auth_code on front channel: {:?}", &params);

    // Get the nonce and state from the session, to validate the request
    let Some(nonce) = session.get::<Nonce>("oauth_nonce")? else {
        // If there is no nonce, something went wrong.  It's likely a stale session
        session.clear();
        return Err(ErrorInternalServerError(
            "Failed to retrieve nonce from session",
        ));
    };
    // one time use.
    session.remove("oauth_nonce");

    // The state value provided in the request is returned in the response
    let _state = CsrfToken::new(params.state.clone());

    // The Authorization Code will be exchanged for the token
    let code = AuthorizationCode::new(params.code.clone());

    // Exchange the auth code for the token.  This does a back channel call to the Auth Server
    let userinfo = exchange_auth_code(code, _state, nonce, &app_state)
        .await
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
    session.insert::<bool>("authenticated", true)?;
    session.insert("display_name", display_name)?;
    session.insert("userinfo", serde_json::to_string_pretty(&userinfo)?)?;

    Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
}
