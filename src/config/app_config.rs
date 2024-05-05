use anyhow::Result;
use log::*;
use serde::Deserialize;
use std::{env, fs::read_to_string};

use super::OidcConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub server_port: u16,
    pub client_id: String,
    pub client_secret: String,
    pub session_ttl: i64,
    pub oidc: OidcConfig,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        let server_port = from_env::<u16>("SERVER_PORT")?.unwrap_or(3000);

        let server_address = env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string());

        let client_id = env::var("OIDC_CLIENT_ID")?;

        let client_secret = env::var("OIDC_CLIENT_SECRET")?;

        let session_ttl = from_env::<i64>("SERVER_SESSION_TTL")?.unwrap_or(1);

        // If an oidc.toml file was provided, load it.  Else just use defaults.
        let oidc_file_name = env::var("OIDC_CONFIG_FILE").unwrap_or("oidc.toml".to_string());
        let oidc = match read_to_string(oidc_file_name) {
            Ok(oidc_file) => {
                info!("Loading OidcConfig from toml file");
                toml::from_str::<OidcConfig>(&oidc_file)?
            }
            _ => {
                info!("Using default OidcConfig");
                OidcConfig::default()
            }
        };

        Ok(Self {
            server_address,
            server_port,
            client_id,
            client_secret,
            session_ttl,
            oidc,
        })
    }
}

fn from_env<T: for<'a> Deserialize<'a>>(key: &str) -> Result<Option<T>> {
    if let Ok(value) = env::var(key) {
        Ok(Some(serde_json::from_str::<T>(&value)?))
    } else {
        Ok(None)
    }
}
