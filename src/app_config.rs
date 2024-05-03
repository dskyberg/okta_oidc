use std::env;
use actix_web::cookie::time::Duration;
use anyhow::Result;
use openidconnect::core::CoreClient;

use crate::auth::oidc_client::oidc_client_setup;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub server_port: u16,
    pub session_timeout: Duration,
    pub okta_app: String,
    pub okta_domain: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub oidc_scopes: Vec<String>,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        let server_port = env::var("SERVER_PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()?;

        let server_address = env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string());

        let minutes = env::var("SESSION_TIMEOUT")
            .unwrap_or("1".to_string())
            .parse::<i64>()
            .expect("Failed to parse session timeout");
        let session_timeout = Duration::minutes(minutes);

        let okta_domain =
            env::var("OKTA_DOMAIN")?;

        let okta_app = env::var("OKTA_APP")?;

        let oidc_client_id =
            env::var("OIDC_CLIENT_ID")?;

        let oidc_client_secret = env::var("OIDC_CLIENT_SECRET")?;

        let oidc_scopes = env::var("OIDC_SCOPES")
            .unwrap_or("email profile".to_string())
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            server_address,
            server_port,
            session_timeout,
            okta_app,
            okta_domain,
            oidc_client_id,
            oidc_client_secret,
            oidc_scopes,
        })
    }

    pub fn oidc_issuer_url(&self) -> String {
        format!(
            "https://{}.oktapreview.com/oauth2/{}",
            &self.okta_domain, &self.okta_app
        )
    }

    pub fn oidc_redirect_url(&self) -> String {
        format!("http://{}:{}/auth", &self.server_address, &self.server_port)
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub oidc_client: CoreClient,
}

impl AppState {
    pub async fn init() -> Result<Self> {

        let config = AppConfig::init()?;
        let oidc_client = oidc_client_setup(&config).await?;

        Ok(Self{config, oidc_client})
    }
}