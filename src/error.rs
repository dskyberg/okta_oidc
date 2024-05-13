use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Server did not return an ID TOKEN")]
    NoIdToken,
    #[error("Keyring error")]
    FailedKeyringFetch,
    #[error("No client_id set in env")]
    NoClientId,
    #[error("No client_secret set in env")]
    NoClientSecret,
    #[error("Token exchange error: {0}")]
    AuthRequest(String),
}
