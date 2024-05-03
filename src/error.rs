use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Server did not return an ID TOKEN")]
    NoIdToken
}