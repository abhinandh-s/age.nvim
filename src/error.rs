use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};

#[derive(Debug)]
pub enum AgeError {
    NvimError(nvim_oxi::Error),
    ApiError(nvim_oxi::api::Error),
    IoError(std::io::Error),
    Other(String),
    Custom(String),
}

impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgeError::NvimError(err) => write!(f, "Neovim Error: {}", err),
            AgeError::ApiError(err) => write!(f, "API Error: {}", err),
            AgeError::IoError(err) => write!(f, "IO Error: {}", err),
            AgeError::Other(msg) => write!(f, "Error: {}", msg),
            AgeError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// Implement `From<AgeError>` for `nvim_oxi::Error`.
 impl From<AgeError> for OxiError {
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
    fn from(err: AgeError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}

impl std::error::Error for AgeError {}

impl From<nvim_oxi::Error> for AgeError {
    fn from(err: nvim_oxi::Error) -> Self {
        AgeError::NvimError(err)
    }
}

impl From<nvim_oxi::api::Error> for AgeError {
    fn from(err: nvim_oxi::api::Error) -> Self {
        AgeError::ApiError(err)
    }
}

impl From<std::io::Error> for AgeError {
    fn from(err: std::io::Error) -> Self {
        AgeError::IoError(err)
    }
}

impl From<&str> for AgeError {
    fn from(msg: &str) -> Self {
        AgeError::Other(msg.to_owned())
    }
}

impl From<Box<dyn std::error::Error>> for AgeError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        // Here, we convert the boxed error into a string and wrap it in AgeError::Other
        AgeError::Other(err.to_string())
    }
}
