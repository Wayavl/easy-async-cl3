#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClImageChannelOrder {
    R,
    A,
    RG,
    RA,
    RGB,
    RGBA,
    BGRA,
    ARGB,
    ABGR,
    Intensity,
    Luminance,
    Rx,
    RGx,
    RGBx,
    Depth,
    sRGB,
    sRGBx,
    sRGBA,
    Unknown(u32),
}

impl From<u32> for ClImageChannelOrder {
    fn from(value: u32) -> Self {
        match value {
            cl3::memory::CL_R => Self::R,
            cl3::memory::CL_A => Self::A,
            cl3::memory::CL_RG => Self::RG,
            cl3::memory::CL_RA => Self::RA,
            cl3::memory::CL_RGB => Self::RGB,
            cl3::memory::CL_RGBA => Self::RGBA,
            cl3::memory::CL_BGRA => Self::BGRA,
            cl3::memory::CL_ARGB => Self::ARGB,
            cl3::memory::CL_ABGR => Self::ABGR,
            cl3::memory::CL_INTENSITY => Self::Intensity,
            cl3::memory::CL_LUMINANCE => Self::Luminance,
            cl3::memory::CL_Rx => Self::Rx,
            cl3::memory::CL_RGx => Self::RGx,
            cl3::memory::CL_RGBx => Self::RGBx,
            cl3::memory::CL_DEPTH => Self::Depth,
            cl3::memory::CL_sRGB => Self::sRGB,
            cl3::memory::CL_sRGBx => Self::sRGBx,
            cl3::memory::CL_sRGBA => Self::sRGBA,
            other => Self::Unknown(other),
        }
    }
}

impl Into<u32> for ClImageChannelOrder {

fn into(self) -> u32 {
        match self {
            ClImageChannelOrder::R => cl3::memory::CL_R,
            ClImageChannelOrder::A => cl3::memory::CL_A,
            ClImageChannelOrder::RG => cl3::memory::CL_RG,
            ClImageChannelOrder::RA => cl3::memory::CL_RA,
            ClImageChannelOrder::RGB => cl3::memory::CL_RGB,
            ClImageChannelOrder::RGBA => cl3::memory::CL_RGBA,
            ClImageChannelOrder::BGRA => cl3::memory::CL_BGRA,
            ClImageChannelOrder::ARGB => cl3::memory::CL_ARGB,
            ClImageChannelOrder::ABGR => cl3::memory::CL_ABGR,
            ClImageChannelOrder::Intensity => cl3::memory::CL_INTENSITY,
            ClImageChannelOrder::Luminance => cl3::memory::CL_LUMINANCE,
            ClImageChannelOrder::Rx => cl3::memory::CL_Rx,
            ClImageChannelOrder::RGx => cl3::memory::CL_RGx,
            ClImageChannelOrder::RGBx => cl3::memory::CL_RGBx,
            ClImageChannelOrder::Depth => cl3::memory::CL_DEPTH,
            ClImageChannelOrder::sRGB => cl3::memory::CL_sRGB,
            ClImageChannelOrder::sRGBx => cl3::memory::CL_sRGBx,
            ClImageChannelOrder::sRGBA => cl3::memory::CL_sRGBA,
            ClImageChannelOrder::Unknown(v) => v,

        }
    }
}
