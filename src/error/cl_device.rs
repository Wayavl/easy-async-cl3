#[derive(Debug)]
pub enum DeviceError {
    GetDataError(i32),
    SubdeviceNotAvailable,
    CouldNotDivideDevice(i32),
    FailedToFormat
}