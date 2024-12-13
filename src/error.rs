use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};

#[derive(Debug)]
pub enum JustError {
    NvimError(nvim_oxi::Error),
    ApiError(nvim_oxi::api::Error),
    IoError(std::io::Error),
    Other(String),
    Custom(String),
}

impl std::fmt::Display for JustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustError::NvimError(err) => write!(f, "Neovim Error: {}", err),
            JustError::ApiError(err) => write!(f, "API Error: {}", err),
            JustError::IoError(err) => write!(f, "IO Error: {}", err),
            JustError::Other(msg) => write!(f, "Error: {}", msg),
            JustError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

// Implement `From<JustError>` for `nvim_oxi::Error`.
 impl From<JustError> for OxiError {
    /// Converts a `JustError` into a `nvim_oxi::Error`.
    ///
    /// This allows the `JustError` to be returned where an `OxiError` is expected, ensuring compatibility
    /// with the Neovim API. It wraps the `JustError` as an `OxiApiError::Other` variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::error::JustError;
    /// use nvim_oxi::Error as OxiError;
    ///
    /// fn example() -> Result<(), OxiError> {
    ///     let error: JustError = JustError::Custom("Something went wrong".to_string());
    ///     Err(OxiError::from(error))
    /// }
    /// ```
    fn from(err: JustError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}

impl std::error::Error for JustError {}

impl From<nvim_oxi::Error> for JustError {
    fn from(err: nvim_oxi::Error) -> Self {
        JustError::NvimError(err)
    }
}

impl From<nvim_oxi::api::Error> for JustError {
    fn from(err: nvim_oxi::api::Error) -> Self {
        JustError::ApiError(err)
    }
}

impl From<std::io::Error> for JustError {
    fn from(err: std::io::Error) -> Self {
        JustError::IoError(err)
    }
}

impl From<&str> for JustError {
    fn from(msg: &str) -> Self {
        JustError::Other(msg.to_owned())
    }
}

impl From<Box<dyn std::error::Error>> for JustError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        // Here, we convert the boxed error into a string and wrap it in JustError::Other
        JustError::Other(err.to_string())
    }
}
