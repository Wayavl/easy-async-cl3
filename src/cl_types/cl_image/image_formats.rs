use crate::cl_types::cl_image::{image_channel_data_type::{ClImageChannelType}, image_channel_order::ClImageChannelOrder};
#[derive(Debug, Clone, Copy )]
pub struct ClImageFormats {
    pub image_channel_order: ClImageChannelOrder,
    pub image_channel_data_type: ClImageChannelType,
}

impl ClImageFormats {
    pub fn rgba_unorm_int8() -> Self {
        Self {
            image_channel_order: ClImageChannelOrder::RGBA,
            image_channel_data_type: ClImageChannelType::UnormInt8,
        }
    }

    pub fn rgba_float() -> Self {
        Self {
            image_channel_order: ClImageChannelOrder::RGBA,
            image_channel_data_type: ClImageChannelType::Float,
        }
    }
}
