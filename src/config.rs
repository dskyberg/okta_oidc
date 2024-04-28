use std::env;

use actix_web::cookie::time::Duration;

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
    pub oidc_username_claim: String,
}

impl AppConfig {
    pub fn init() -> Self {
        let server_port = env::var("SERVER_PORT").unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("Failed to parse port");
        
        let server_address = env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string());
       
        let minutes = env::var("SESSION_TIMEOUT")
        .unwrap_or("1".to_string())
        .parse::<i64>()
        .expect("Failed to parse session timeout");
        let session_timeout = Duration::minutes(minutes);

        let okta_domain = env::var("OKTA_DOMAIN").expect("Missing the OKTA_DOMAIN environment variable");
        
        let okta_app = env::var("OKTA_APP").expect("Missing the OKTA_APP environment variable");

        let oidc_client_id =
        env::var("OIDC_CLIENT_ID").expect("Missing the OIDC_CLIENT_ID environment variable.");
        
        let oidc_client_secret = env::var("OIDC_CLIENT_SECRET")
        .expect("Missing the OIDC_CLIENT_SECRET environment variable.");

        let oidc_scopes = env::var("OIDC_SCOPES").unwrap_or("email profile".to_string())
        .split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
        
        let oidc_username_claim = env::var("OIDC_USERNAME").unwrap_or("name".to_string());

        Self{
            server_address, server_port, session_timeout, okta_app, okta_domain, oidc_client_id, oidc_client_secret, oidc_scopes, oidc_username_claim
        }

    }

    pub fn oidc_issuer_url(&self) -> String {
        format!("https://{}.oktapreview.com/oauth2/{}", &self.okta_domain, &self.okta_app)
    }

    pub fn oidc_redirect_url(&self) -> String {
        format!("http://{}:{}/auth", &self.server_address, &self.server_port)
    }
}