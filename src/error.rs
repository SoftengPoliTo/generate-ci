use thiserror::Error;

///Error types
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// Wrong expansion to home directory.
    #[error("Wrong expansion")]
    WrongExpandUser,
    /// Impossible to canonicalize this path.
    #[error("Canonicalization failure")]
    CanonicalPath,
    /// Path does not exists .
    #[error("Non-existent Path")]
    PathNotExist,
    /// Unable to retrieve the home directory.
    #[error("Home directory failure")]
    HomeDir,
    /// A general utf-8 conversion error.
    #[error("Utf-8 error")]
    UTF8Check,
    /// License not found.
    #[error("License not found")]
    NoLicense,
    /// Non-existent home directory
    #[error("Non-existent home directory")]
    NoDirExists,
    /// Template not found
    #[error("Template not found")]
    TemplateNotFound,
    /// Context not found
    #[error("Context not found")]
    NoContext,
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
