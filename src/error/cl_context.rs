#[derive(Debug)]
pub enum ContextError {
    ErrorCreatingContext(i32),
    GetDataError(i32),
    FailedToFormat,
}