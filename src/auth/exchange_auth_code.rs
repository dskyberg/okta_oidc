use anyhow::Result;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthenticationContextClass, AuthorizationCode, CsrfToken, Nonce};
use serde_json::Value;
use tracing::*;
use tracing_attributes::instrument;

use crate::error::Error;
use crate::AppState;

fn acr_verifier(acr: Option<&AuthenticationContextClass>) -> Result<(), String> {
    if let Some(acr) = acr {
        info!("Received acr: {:?}", acr);
    } else {
        info!("No acr in token");
    }
    Ok(())
}

#[allow(dead_code)]
fn amr_verifier(acr: Option<String>) -> Result<(), String> {
    if let Some(acr) = acr {
        info!("Received acr: {:?}", acr);
    } else {
        info!("No acr in token");
    }
    Ok(())
}
/// The user is redirected to the Authorization Server by the `/login` endpoint.
/// The Authorization Server authenticates the user, and returns a auth code.
/// This endpoint exchanges the auth code for a token response that contains the access_token and id_token.
#[instrument(skip(code, _state, nonce, app_state))]
pub async fn exchange_auth_code(
    code: AuthorizationCode,
    _state: CsrfToken,
    nonce: Nonce,
    app_state: &AppState,
) -> Result<Value> {
    info!("Exchanging auth code: {}", code.secret());
    // Exchange the code with a token.
    let token_response = app_state
        .oidc_client
        .exchange_code(code)
        .request_async(async_http_client)
        .await?;

    info!("Verifying token");
    let id_token_verifier = app_state
        .oidc_client
        .id_token_verifier()
        .require_audience_match(true)
        .require_audience_match(true)
        .set_auth_context_verifier_fn(acr_verifier);

    info!("extracting claims");
    let id_token_claims = token_response
        .extra_fields()
        .id_token()
        .ok_or(Error::NoIdToken)?
        .claims(&id_token_verifier, &nonce)?;

    // Convert back to raw JSON to simplify extracting configurable claims
    let userinfo = serde_json::to_value(id_token_claims).unwrap();
    info!(
        "Exchanged code for token response on back channel: \n{}",
        serde_json::to_string_pretty(&userinfo).unwrap()
    );

    Ok(userinfo)
}
