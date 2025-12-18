#[derive(Debug)]
pub enum PlatformError {
    GetDataError(i32),
    GetIdError(i32),
    CouldNotFindPlatform,
    CouldNotGetDevice(i32),
    FailedToFormat
}