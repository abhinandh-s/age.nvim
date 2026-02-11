use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};

#[derive(Debug)]
pub enum Error {
    Nvim(nvim_oxi::Error),
    Api(nvim_oxi::api::Error),
    Io(std::io::Error),
    Other(String),
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Nvim(err) => write!(f, "{err}"),
            Error::Api(err) => write!(f, "API Error: {err}"),
            Error::Io(err) => write!(f, "IO Error: {err}"),
            Error::Other(err) => write!(f, "Error: {err}"),
            Error::Custom(err) => write!(f, "Error: {err}"),
        }
    }
}

// Implement `From<AgeError>` for `nvim_oxi::Error`.
impl From<Error> for OxiError {
    /// Converts a `AgeError` into a `nvim_oxi::Error`.
    ///
    /// This allows the `AgeError` to be returned where an `OxiError` is expected, ensuring compatibility
    /// with the Neovim API. It wraps the `AgeError` as an `OxiApiError::Other` variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::error::AgeError;
    /// use nvim_oxi::Error as OxiError;
    ///
    /// fn example() -> Result<(), OxiError> {
    ///     let error: AgeError = AgeError::Custom("Something went wrong".to_string());
    ///     Err(OxiError::from(error))
    /// }
    /// ```
    fn from(err: Error) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{err}")))
    }
}

impl std::error::Error for Error {}

impl From<nvim_oxi::Error> for Error {
    fn from(err: nvim_oxi::Error) -> Self {
        Error::Nvim(err)
    }
}

impl From<nvim_oxi::api::Error> for Error {
    fn from(err: nvim_oxi::api::Error) -> Self {
        Error::Api(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Other(msg.to_owned())
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Other(err.to_string())
    }
}

impl From<age::DecryptError> for Error {
    fn from(value: age::DecryptError) -> Self {
        Error::Other(value.to_string())
    }
}

impl From<age::EncryptError> for Error {
    fn from(value: age::EncryptError) -> Self {
        Error::Other(value.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Error::Other(value.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error::Other(value.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Error::Other(value.to_string())
    }
}
