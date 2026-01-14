use crate::cl_types::cl_image::{image_channel_data_type::{ClImageChannelType}, image_channel_order::ClImageChannelOrder};
#[derive(Debug, Clone, Copy )]
pub struct ClImageFormats {
    pub image_channel_order: ClImageChannelOrder,
    pub image_channel_data_type: ClImageChannelType,
}

