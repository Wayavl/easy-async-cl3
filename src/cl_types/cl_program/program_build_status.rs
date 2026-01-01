pub enum ProgramBuildStatus {
    BuildNone,
    BuildError,
    BuildSuccess,
    BuildInProgress,
    Unknown(i32),
}

impl From<i32> for ProgramBuildStatus {
    fn from(value: i32) -> Self {
        match value {
            cl3::program::CL_BUILD_ERROR => Self::BuildError,
            cl3::program::CL_BUILD_IN_PROGRESS => Self::BuildInProgress,
            cl3::program::CL_BUILD_NONE => Self::BuildSuccess,
            cl3::program::CL_BUILD_SUCCESS => Self::BuildSuccess,
            _=> Self::Unknown(value)
        }
    }
}