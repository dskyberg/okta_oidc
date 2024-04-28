//!
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{ web, Responder,error::ErrorInternalServerError,};
use log::info;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthorizationCode, CsrfToken};

pub use oidc_client::OidcClient;
pub use auth_request::AuthRequest;

pub mod oidc_client;
pub mod auth_request;

pub async fn auth(
    session: Session,
    oidc: web::Data<OidcClient>,
    params: web::Query<AuthRequest>,
) -> actix_web::Result<impl Responder> {

    
    info!("Recieved auth_code on front channel: {:?}", &params);

    let _state = CsrfToken::new(params.state.clone());
    let code = AuthorizationCode::new(params.code.clone());
    
    // Exchange the code with a token.
    let token_response = oidc
        .client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .map_err(ErrorInternalServerError)?;

    
    let id_token_verifier = oidc.client.id_token_verifier();
    let id_token_claims = token_response
        .extra_fields()
        .id_token()
        .ok_or(ErrorInternalServerError(
            "Server did not return an ID token",
        ))?
        .claims(&id_token_verifier, &oidc.nonce)
        .map_err(ErrorInternalServerError)?;

    // Convert back to raw JSON to simplify extracting configurable claims
    let userinfo = serde_json::to_value(id_token_claims).unwrap();
    info!("Exchanged code for token response on back channel: \n{}", serde_json::to_string_pretty(&userinfo).unwrap());

    let username = if let Some(claim) = &oidc.username_claim {
        userinfo[claim].as_str()
    } else {
        userinfo
            .get("preferred_username")
            .or(userinfo.get("upn"))
            .or(userinfo.get("email"))
            .and_then(|v| v.as_str())
    }
    .unwrap_or("");

    session.insert("username", username)?;
    session.insert("userinfo", serde_json::to_string_pretty(&userinfo)?)?;

    Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
}

