use anyhow::Result;
use openidconnect::core::CoreClient;

use crate::auth::oidc_client::oidc_client_setup;

use super::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub oidc_client: CoreClient,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let config = AppConfig::init()?;
        let oidc_client = oidc_client_setup(&config).await?;

        Ok(Self {
            config,
            oidc_client,
        })
    }
}
