use anyhow::Result;
use openidconnect::core::CoreProviderMetadata;

use crate::auth::oidc_client::{oidc_client_setup, OktaClient};

use super::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub oidc_client: OktaClient,
    pub oidc_metadata: CoreProviderMetadata,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let config = AppConfig::init()?;
        let (oidc_client, oidc_metadata) = oidc_client_setup(&config).await?;

        Ok(Self {
            config,
            oidc_client,
            oidc_metadata,
        })
    }

    pub fn claims(&self) -> Vec<String> {
        let mut result = Vec::<String>::new();
        if let Some(claims) = self.oidc_metadata.claims_supported() {
            for claim in claims {
                result.push(claim.to_string())
            }
        }

        result
    }

    pub fn scopes(&self) -> Vec<String> {
        let mut result = Vec::<String>::new();
        if let Some(scopes) = self.oidc_metadata.scopes_supported() {
            for scope in scopes {
                result.push(scope.to_string())
            }
        }

        result
    }
}
