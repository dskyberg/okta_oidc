#[cfg(feature = "keyring")]
pub use keyring_creds::client_creds;

#[cfg(not(feature = "keyring"))]
pub use env_creds::client_creds;

#[cfg(not(feature = "keyring"))]
mod env_creds;
#[cfg(feature = "keyring")]
mod keyring_creds;
