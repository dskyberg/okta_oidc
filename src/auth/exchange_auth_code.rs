use anyhow::Result;
use openidconnect::{
    AuthenticationContextClass, AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse,
    PkceCodeVerifier,
};
use serde_json::Value;
use tracing::*;
use tracing_attributes::instrument;

use crate::auth::oidc_client::http_client;
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
    _state: Option<CsrfToken>,
    nonce: Nonce,
    app_state: &AppState,
    pkce_verifier: Option<PkceCodeVerifier>,
) -> Result<Value> {
    info!("Exchanging auth code: {}", code.secret());
    // Exchange the code with a token.
    let mut token_request = app_state.oidc_client.inner().exchange_code(code)?;

    if let Some(pkce_verifier) = pkce_verifier {
        token_request = token_request.set_pkce_verifier(pkce_verifier);
    }

    let request_client = http_client()?;

    let token_response = token_request
        .request_async(&request_client)
        .await
        .map_err(|e| {
            error!("{}", e.to_string());
            Error::AuthRequest(e.to_string())
        })?;

    let _access_token = token_response.access_token();

    // Use the openidconnect lib to establish a token verifier
    let id_token_verifier = app_state
        .oidc_client
        .inner()
        .id_token_verifier()
        .require_audience_match(true)
        .enable_signature_check()
        .set_auth_context_verifier_fn(acr_verifier);

    info!("Verifying token and extracting the id_token");
    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or(Error::NoIdToken)?
        .claims(&id_token_verifier, &nonce)?;

    // Convert back to raw JSON to simplify extracting configurable claims
    let userinfo = serde_json::to_value(id_token).unwrap();
    info!(
        "Exchanged code for token response on back channel: \n{}",
        serde_json::to_string_pretty(&userinfo).unwrap()
    );

    Ok(userinfo)
}
