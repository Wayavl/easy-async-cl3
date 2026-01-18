use crate::error::{api_error::ApiError, wrapper_error::WrapperError};

pub mod api_error;
pub mod wrapper_error;

/// # ClError
/// 
/// The main error type for this library.
/// 
/// Errors can come from two sources:
/// - `Api`: Errors from the underlying OpenCL API (e.g., invalid arguments, out of memory)
/// - `Wrapper`: Errors from this library's wrapper code (e.g., failed conversions, missing resources)
/// 
/// Both variants implement `Display` and `Error` for easy error handling.
#[derive(Debug)]
pub enum ClError {
    Api(ApiError),
    Wrapper(WrapperError)
}

impl std::fmt::Display for ClError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClError::Api(err) => write!(f, "OpenCL API Error: {}", err),
            ClError::Wrapper(err) => write!(f, "Library Wrapper Error: {:?}", err),
        }
    }
}

impl std::error::Error for ClError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClError::Api(err) => Some(err),
            ClError::Wrapper(_) => None, // WrapperError should also implement Error if needed
        }
    }
}