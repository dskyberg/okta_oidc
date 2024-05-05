use anyhow::Result;
use std::env;

use crate::error::Error;

/// Fetches client id and secret either from keyring or the env
/// Returns (client_id: String, client_secret: String)
pub fn client_creds() -> Result<(String, String)> {
    let client_id = env::var("OIDC_CLIENT_ID").map_err(|_| Error::NoClientId)?;
    let client_secret = env::var("OIDC_CLIENT_SECRET").map_err(|_| Error::NoClientSecret)?;
    Ok((client_id, client_secret))
}
