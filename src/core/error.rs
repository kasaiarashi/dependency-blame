use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DependencyBlameError {
    #[error("Git repository not found at path: {0}")]
    GitRepoNotFound(PathBuf),

    #[error("Dependency file not found: {0}")]
    DependencyFileNotFound(String),

    #[error("Failed to parse dependency file '{file}': {reason}")]
    ParseError { file: String, reason: String },

    #[error("Dependency '{0}' not found in project")]
    DependencyNotFound(String),

    #[error("Unsupported ecosystem")]
    UnsupportedEcosystem,

    #[error("Could not detect ecosystem from path: {0}")]
    EcosystemDetectionFailed(PathBuf),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DependencyBlameError>;
