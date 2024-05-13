use anyhow::Result;
use openidconnect::core::{
    CoreAuthDisplay, CoreAuthPrompt, CoreClient, CoreErrorResponseType, CoreGenderClaim,
    CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreProviderMetadata, CoreRevocableToken,
    CoreRevocationErrorResponse, CoreTokenIntrospectionResponse, CoreTokenResponse,
};
use openidconnect::{
    reqwest, Client, EmptyAdditionalClaims, EndpointMaybeSet, EndpointNotSet, EndpointSet,
    StandardErrorResponse,
};
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use tracing::instrument::Instrumented;
use tracing::*;

use crate::AppConfig;

/// OpenID Connect Core client that is established from Okta metadata.
pub type OktaClient<
    HasAuthUrl = EndpointSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointMaybeSet,
    HasUserInfoUrl = EndpointMaybeSet,
> = Instrumented<
    Client<
        EmptyAdditionalClaims,
        CoreAuthDisplay,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJsonWebKey,
        CoreAuthPrompt,
        StandardErrorResponse<CoreErrorResponseType>,
        CoreTokenResponse,
        CoreTokenIntrospectionResponse,
        CoreRevocableToken,
        CoreRevocationErrorResponse,
        HasAuthUrl,
        HasDeviceAuthUrl,
        HasIntrospectionUrl,
        HasRevocationUrl,
        HasTokenUrl,
        HasUserInfoUrl,
    >,
>;

pub fn http_client() -> Result<reqwest::Client> {
    let client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    Ok(client)
}

#[instrument(skip(app_config))]
pub async fn oidc_client_setup(
    app_config: &AppConfig,
) -> Result<(OktaClient, CoreProviderMetadata)> {
    info!(
        "Fetching {}/.well-known/openid-configuration",
        app_config.oidc.issuer_url()
    );

    let issuer_url = IssuerUrl::new(app_config.oidc.issuer_url())?;
    let redirect_url = RedirectUrl::new(app_config.oidc.redirect_url())?;

    let request_client = http_client()?;
    let provider_metadata =
        CoreProviderMetadata::discover_async(issuer_url, &request_client).await?;
    info!("Provider Metadata: {:#?}", &provider_metadata);

    // Set up the config for the OAuth2 process.
    let client = CoreClient::from_provider_metadata(
        provider_metadata.clone(),
        ClientId::new(app_config.client_id.clone()),
        Some(ClientSecret::new(app_config.client_secret.clone())),
    )
    .set_redirect_uri(redirect_url)
    .instrument(tracing::info_span!("oidc"));

    Ok((client, provider_metadata))
}
