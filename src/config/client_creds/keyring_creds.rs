use anyhow::Result;
use keyring::Entry;

use crate::error::Error;

pub fn client_creds() -> Result<(String, String)> {
    let entry = Entry::new("okta_oidc", "default")?;
    let password = entry.get_password()?;
    password
        .split_once(':')
        .map(|(id, secret)| (id.to_owned(), secret.to_owned()))
        .ok_or(Error::FailedKeyringFetch.into())
}
