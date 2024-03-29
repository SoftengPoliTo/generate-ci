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
    Utf8Check,
    /// Invalid license
    #[error("Invalid license")]
    InvalidLicense(license::ParseError),
    /// A more generic I/O error.
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    /// A minijinja error.
    #[error("Minijinja error")]
    Minijinja(#[from] minijinja::Error),
}

impl From<license::ParseError> for Error {
    fn from(e: license::ParseError) -> Error {
        Error::InvalidLicense(e)
    }
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
