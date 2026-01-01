use cl3::ext::{CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT, CL_PROGRAM_BINARY_TYPE_EXECUTABLE, CL_PROGRAM_BINARY_TYPE_INTERMEDIATE, CL_PROGRAM_BINARY_TYPE_LIBRARY, CL_PROGRAM_BINARY_TYPE_NONE};

pub enum ProgramBinaryType {
    BinaryTypeNone,
    BinaryTypeCompiledObject,
    BinaryTypeLibrary,
    BinaryTypeExecutable,
    BinaryTypeIntermediate,
    Unknown(u32)
}

impl From<u32> for ProgramBinaryType {
    fn from(value: u32) -> Self {
        match value {
            CL_PROGRAM_BINARY_TYPE_NONE => Self::BinaryTypeNone,
            CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT => Self::BinaryTypeCompiledObject,
            CL_PROGRAM_BINARY_TYPE_LIBRARY => Self::BinaryTypeLibrary,
            CL_PROGRAM_BINARY_TYPE_EXECUTABLE => Self::BinaryTypeExecutable,
            CL_PROGRAM_BINARY_TYPE_INTERMEDIATE => Self::BinaryTypeIntermediate,
            _=> Self::Unknown(value)
        }
    }
}