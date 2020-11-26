use thiserror::Error;

#[derive(Debug, Error)]
pub enum BooruError {
    #[error("error sending HTTP request {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("error deserializing post json {0}")]
    Deserialization(#[from] serde_json::Error)
}