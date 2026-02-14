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

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Other(if msg.to_lowercase().starts_with("error") {
            msg.to_owned()
        } else {
            format!("Error: {}", msg)
        })
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Other(err.to_string())
    }
}

macro_rules! impl_err {
    ($($from:path),* $(,)?) => {
        impl std::error::Error for Error {}
        $(
            impl From<$from> for Error {
                fn from(err: $from) -> Self {
                    let string = err.to_string();
                    Error::Other(
                        if string.to_lowercase().starts_with("error") {
                            string
                        } else {
                            format!("Error: {}", string)
                        }
                    )
                }
            }
        )*
    }
}

impl_err![
    nvim_oxi::Error,
    std::io::Error,
    nvim_oxi::api::Error,
    std::env::VarError,
    std::str::Utf8Error,
    std::string::FromUtf8Error,
    age::EncryptError,
    age::DecryptError,
];
