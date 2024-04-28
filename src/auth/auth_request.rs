use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
