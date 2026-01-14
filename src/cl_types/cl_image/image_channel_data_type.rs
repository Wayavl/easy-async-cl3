#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ClImageChannelType {
    SnormInt8,
    SnormInt16,

    UnormInt8,
    UnormInt16,
    UnormShort565,
    UnormShort555,
    UnormInt101010,

    SignedInt8,
    SignedInt16,
    SignedInt32,

    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,

    HalfFloat,
    Float,

    Unknown(u32),
}

impl From<u32> for ClImageChannelType {
    fn from(value: u32) -> Self {
        match value {
            cl3::memory::CL_SNORM_INT8 => Self::SnormInt8,
            cl3::memory::CL_SNORM_INT16 => Self::SnormInt16,
            cl3::memory::CL_UNORM_INT8 => Self::UnormInt8,
            cl3::memory::CL_UNORM_INT16 => Self::UnormInt16,
            cl3::memory::CL_UNORM_SHORT_565 => Self::UnormShort565,
            cl3::memory::CL_UNORM_SHORT_555 => Self::UnormShort555,
            cl3::memory::CL_UNORM_INT_101010 => Self::UnormInt101010,
            cl3::memory::CL_SIGNED_INT8 => Self::SignedInt8,
            cl3::memory::CL_SIGNED_INT16 => Self::SignedInt16,
            cl3::memory::CL_SIGNED_INT32 => Self::SignedInt32,
            cl3::memory::CL_UNSIGNED_INT8 => Self::UnsignedInt8,
            cl3::memory::CL_UNSIGNED_INT16 => Self::UnsignedInt16,
            cl3::memory::CL_UNSIGNED_INT32 => Self::UnsignedInt32,
            cl3::memory::CL_HALF_FLOAT => Self::HalfFloat,
            cl3::memory::CL_FLOAT => Self::Float,
            other => Self::Unknown(other),
        }
    }
}

impl Into<u32> for ClImageChannelType {
    fn into(self) -> u32 {
        match self {
            Self::SnormInt8 => cl3::memory::CL_SNORM_INT8,
            Self::SnormInt16 => cl3::memory::CL_SNORM_INT16,
            Self::UnormInt8 => cl3::memory::CL_UNORM_INT8,
            Self::UnormInt16 => cl3::memory::CL_UNORM_INT16,
            Self::UnormShort565 => cl3::memory::CL_UNORM_SHORT_565,
            Self::UnormShort555 => cl3::memory::CL_UNORM_SHORT_555,
            Self::UnormInt101010 => cl3::memory::CL_UNORM_INT_101010,
            Self::SignedInt8 => cl3::memory::CL_SIGNED_INT8,
            Self::SignedInt16 => cl3::memory::CL_SIGNED_INT16,
            Self::SignedInt32 => cl3::memory::CL_SIGNED_INT32,
            Self::UnsignedInt8 => cl3::memory::CL_UNSIGNED_INT8,
            Self::UnsignedInt16 => cl3::memory::CL_UNSIGNED_INT16,
            Self::UnsignedInt32 => cl3::memory::CL_UNSIGNED_INT32,
            Self::HalfFloat => cl3::memory::CL_HALF_FLOAT,
            Self::Float => cl3::memory::CL_FLOAT,
            Self::Unknown(v) => v,
        }
    }
}

