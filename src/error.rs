use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};
use std::io::Error as IoError;
use thiserror::Error;

/// A custom error type for the Just plugin.
#[derive(Debug, Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("Neovim API error: {0}")]
    Api(#[from] OxiApiError),

    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<PluginError> for OxiError {
    /// Converts a `PluginError` into a `nvim_oxi::Error`.
    ///
    /// This allows the `PluginError` to be returned where an `OxiError` is expected, ensuring compatibility
    /// with the Neovim API. It wraps the `PluginError` as an `OxiApiError::Other` variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::error::PluginError;
    /// use nvim_oxi::Error as OxiError;
    ///
    /// fn example() -> Result<(), OxiError> {
    ///     let error: PluginError = PluginError::Custom("Something went wrong".to_string());
    ///     Err(OxiError::from(error))
    /// }
    /// ```
    fn from(err: PluginError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}


#[derive(Debug)]
pub enum JustError {
    NvimError(nvim_oxi::Error), // Already implemented for nvim_oxi::Error
    ApiError(nvim_oxi::api::Error), // New variant for nvim_oxi::api::Error
    IoError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for JustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JustError::NvimError(err) => write!(f, "Neovim Error: {}", err),
            JustError::ApiError(err) => write!(f, "API Error: {}", err),
            JustError::IoError(err) => write!(f, "IO Error: {}", err),
            JustError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for JustError {}

impl From<nvim_oxi::Error> for JustError {
    fn from(err: nvim_oxi::Error) -> Self {
        JustError::NvimError(err)
    }
}

impl From<nvim_oxi::api::Error> for JustError { // Implement From for api::Error
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
        JustError::Other(msg.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for JustError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        // Here, we convert the boxed error into a string and wrap it in JustError::Other
        JustError::Other(err.to_string())
    }
}
