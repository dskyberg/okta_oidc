use serde::Deserialize;

/// In order to simply manage the TOML file, to only include what you need, this struct uses serde
/// default functions.  Since `serde(default)` does not accept closures, functions are defined for
/// each default value.
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct OidcConfig {
    pub server_address: String,
    pub server_port: u16,
    pub app: String,
    pub domain: String,
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    #[serde(default = "default_amrs")]
    pub amrs: Vec<String>,
    #[serde(default = "default_bool")]
    pub verify_aud: bool,
    #[serde(default = "default_bool")]
    pub use_par: bool,
    #[serde(default = "default_bool")]
    pub use_pkce: bool,
}

/// Default scopes for Deserialize
fn default_scopes() -> Vec<String> {
    vec![String::from("email"), String::from("profile")]
}

/// Default AMRs for Deserialize
fn default_amrs() -> Vec<String> {
    vec![
        String::from("mfa"),
        String::from("user"),
        String::from("hwk"),
        String::from("mfa"),
    ]
}

/// Default verify for Deserialize
fn default_bool() -> bool {
    false
}

impl OidcConfig {
    pub fn issuer_url(&self) -> String {
        format!(
            "https://{}.oktapreview.com/oauth2/{}",
            &self.domain, &self.app
        )
    }

    pub fn redirect_url(&self) -> String {
        format!("http://{}:{}/auth", &self.server_address, &self.server_port)
    }
}

impl Default for OidcConfig {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1".to_string(),
            server_port: 3000,
            app: "default".to_string(),
            domain: "dskyberg".to_string(),
            scopes: default_scopes(),
            amrs: default_amrs(),
            verify_aud: false,
            use_par: false,
            use_pkce: false,
        }
    }
}
