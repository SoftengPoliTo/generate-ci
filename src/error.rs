use thiserror::Error;

///Error types
#[derive(Debug, Error)]
pub enum Error {
    /// Unable to retrieve the home directory.
    #[error("Home directory failure")]
    HomeDir,
    /// Unable to remove a prefix from a path.
    #[error("Unable to remove path prefix")]
    StripPrefix(#[from] std::path::StripPrefixError),
    /// Directory not found.
    #[error("Directory not found")]
    NoDirectory,
    /// A general utf-8 conversion error.
    #[error("Utf-8 error")]
    UTF8Check,
    /// License not found.
    #[error("License not found")]
    NoLicense,
    /// Invalid license
    #[error("Invalid license")]
    InvalidLicense,
    /// A more generic I/O error.
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    /// A minijinja error.
    #[error("Minijinja error")]
    Minijinja(#[from] minijinja::Error),
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
