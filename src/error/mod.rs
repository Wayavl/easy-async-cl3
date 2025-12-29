use crate::error::{api_error::ApiError, wrapper_error::WrapperError};

pub mod api_error;
pub mod wrapper_error;

#[derive(Debug)]
pub enum ClError {
    Api(ApiError),
    Wrapper(WrapperError)
}