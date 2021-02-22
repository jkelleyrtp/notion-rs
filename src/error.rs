pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No TokenV2 supplied, client is not logged in.")]
    NotLoggedIn,

    #[error("Invalid Credentials were provided")]
    InvalidCredentials,

    #[error("Error sending packet ")]
    PostError,

    #[error("Failed parsing Notion's UUID format")]
    UuidParseError(#[from] uuid::Error),

    #[error("Failed reading/writing from disk")]
    IoError(#[from] std::io::Error),

    #[error("Failed fetching/decoding data")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error parsing data")]
    SerdeError(#[from] serde_json::Error),
}
