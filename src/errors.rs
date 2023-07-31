use thiserror::Error;

/// ReciteError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum ribasomeServerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    EnvVariableError(#[from] std::env::VarError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    JsonReadError(#[from] crate::csv_ops::JsonReadError),
}

pub type Result<T> = color_eyre::eyre::Result<T, ribasomeServerError>;
