#[derive(Debug)]
pub enum WrapperError {
    FormatterFailed,
    DefaultPlatformNotFound,
    SubdeviceNotAvailableForThisDevice,
    DeviceNotFoundInProgram,
    FailedToConvertStrToCString,
}
