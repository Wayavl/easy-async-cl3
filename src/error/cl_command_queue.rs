#[derive(Debug)]
pub enum CommandQueueError {
    CommandQueueFailed(i32),
    CommandQueueWithPropertiesFailed(i32),
    QueryDataError(i32),
    FailedToFormat,
}