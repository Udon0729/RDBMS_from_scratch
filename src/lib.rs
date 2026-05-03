// ライブラリのルートモジュール。
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

/// DB version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Database result type
pub type Result<T> = std::result::Result<T, Error>;

/// Database error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL parsing error: {0}")]
    Parse(String),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
