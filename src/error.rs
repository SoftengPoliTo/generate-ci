use thiserror::Error;

///Error types
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// This error is due to the fact that the crate expanduser failed its path extension operation.
    #[error("Error when expanding `~` in the home directory!")]
    ExpandUser,
    /// Error during canonicalization, probably the path entered is nonexistent or a non-final path component is not a directory.
    #[error("Error during canonicalization!")]
    CanonicalPath,
    /// Error related to the existence of the path to be verified.
    #[error("Error: path entered does not exist")]
    PathNotExist,
    /// Unable to retrieve the home directory from this file system!
    #[error("Error: unable to retrieve home directory")]
    HomeDir,
    /// This conversion may involve checking the validity of UTF-8.
    /// Note that the validation is performed because non-UTF-8 strings are perfectly valid for some operating systems.
    #[error("Error: UTF-8 validity check failed for this operating system.")]
    UTF8Check,
    /// This error returns None if the path ends in .. .
    #[error("Error: filename not found")]
    FileNameNotFound,
    /// The license entered was not found.
    #[error("Cannot find license")]
    NoLicense,
    /// Probably a directory entered in the path does not exist.
    /// Check the manual for possible other errors.
    #[error("Cannot find a dir in the path")]
    NoDirExists,
    /// The requested template has not been previously uploaded
    #[error("Template not found")]
    TemplateNotFound,
    /// The render function cannot return a string from the inserted context
    #[error("Template not found")]
    NoContext,
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
