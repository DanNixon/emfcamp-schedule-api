#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error {0}")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
