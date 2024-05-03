use anyhow::Result;
use log::info;
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};

use crate::AppConfig;

pub async fn oidc_client_setup(app_config: &AppConfig) -> Result<CoreClient> {
    info!(
        "Fetching {}/.well-known/openid-configuration",
        app_config.oidc_issuer_url()
    );

    let issuer_url = IssuerUrl::new(app_config.oidc_issuer_url())?;
    let redirect_url = RedirectUrl::new(app_config.oidc_redirect_url())?;

    let provider_metadata =
        CoreProviderMetadata::discover_async(issuer_url, async_http_client).await?;

    // Set up the config for the OAuth2 process.
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(app_config.oidc_client_id.clone()),
        Some(ClientSecret::new(app_config.oidc_client_secret.clone())),
    )
    .set_redirect_uri(redirect_url);

    Ok(client)
}
