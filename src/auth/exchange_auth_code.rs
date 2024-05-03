//!
use log::info;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthorizationCode, CsrfToken, Nonce};
use serde_json::Value;
use anyhow::Result;

use crate::app_config::AppState;
use crate::error::Error;


/// The user is redirected to the Authorization Server by the `/login` endpoint.
/// The Authorization Server authenticates the user, and returns a auth code.
/// This endpoint exchanges the auth code for a token response that contains the access_token and id_token.
pub async fn exchange_auth_code(
    code: AuthorizationCode,
    _state: CsrfToken,
    nonce: Nonce,
    app_state: &AppState,
) -> Result<Value> {

    // Exchange the code with a token.
    let token_response = app_state.oidc_client
        .exchange_code(code)
        .request_async(async_http_client)
        .await?;

    let id_token_verifier = app_state.oidc_client.id_token_verifier();
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
