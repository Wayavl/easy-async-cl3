#[derive(Debug)]
pub enum PlatformError {
    FormatError(i32),
    GetIdError(i32),
}