use log::info;
use openidconnect::core::{CoreClient, CoreProviderMetadata, CoreResponseType};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AuthenticationFlow, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
};

use crate::AppConfig;

#[derive(Clone, Debug)]
pub struct OidcClient {
    pub client: CoreClient,
    pub authorize_url: String,
    pub nonce: Nonce,
    pub username_claim: Option<String>,
}

impl OidcClient {
    pub async fn setup(app_config: &AppConfig) -> Self {
    
        info!("Fetching {}/.well-known/openid-configuration", app_config.oidc_issuer_url());
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(app_config.oidc_issuer_url()).expect("Invalid issuer URL"),
            async_http_client,
        )
        .await
        .expect("Failed to discover OpenID Provider");
    
        // Set up the config for the OAuth2 process.
        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(app_config.oidc_client_id.clone()),
            Some(ClientSecret::new(app_config.oidc_client_secret.clone())),
        )
        .set_redirect_uri(
            RedirectUrl::new(app_config.oidc_redirect_url()).expect("Invalid redirect URL"),
        );
    
        // Generate the authorization URL to which we'll redirect the user.
        let mut auth_client = client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );
        for scope in &app_config.oidc_scopes {
            auth_client = auth_client.add_scope(Scope::new(scope.to_owned()));
        }
        let (authorize_url, _csrf_state, nonce) = auth_client.url();
    
        OidcClient {
            client,
            authorize_url: authorize_url.to_string(),
            nonce,
            username_claim: Some(app_config.oidc_username_claim.clone())
        }
    }
    
}