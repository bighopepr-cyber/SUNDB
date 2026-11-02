use thiserror::Error;

#[derive(Debug, Error)]
pub enum SunDbError {
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("resource not found: {0}")]
    ResourceNotFound(String),
    #[error("storage failure: {0}")]
    StorageFailure(String),
    #[error("transaction aborted: {0}")]
    TransactionAborted(String),
    #[error("security violation: {0}")]
    SecurityViolation(String),
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, SunDbError>;
