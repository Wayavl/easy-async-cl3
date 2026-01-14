use std::{os::raw::c_void, ptr::null_mut};

use cl3::ext::cl_image_desc;

use crate::cl_types::cl_image::image_type::ClImageType;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClImageDesc {
    pub image_type: ClImageType,
    pub image_width: Option<usize>,
    pub image_height: Option<usize>,
    pub image_depth: Option<usize>,
    pub image_array_size: Option<usize>,
    pub image_row_pitch: Option<usize>,
    pub image_slice_pitch: Option<usize>,
    pub num_mip_levels: Option<u32>,
    pub num_samples: Option<u32>,
    pub buffer: Option<*mut c_void>,
}

impl Default for ClImageDesc {
    fn default() -> Self {
        Self {
            image_type: ClImageType::Image1D,
            image_width: None,
            image_height: None,
            image_depth: None,
            image_array_size: None,
            image_row_pitch: None,
            image_slice_pitch: None,
            num_mip_levels: None,
            num_samples: None,
            buffer: None
        }
    }
}

impl Into<cl_image_desc> for ClImageDesc {
    fn into(self) -> cl_image_desc {
        cl_image_desc {
            image_type: self.image_type.into(),
            image_width: self.image_height.unwrap_or(0),
            image_height: self.image_height.unwrap_or(0),
            image_depth:  self.image_depth.unwrap_or(0),
            image_array_size: self.image_array_size.unwrap_or(0),
            image_row_pitch: self.image_row_pitch.unwrap_or(0),
            image_slice_pitch: self.image_slice_pitch.unwrap_or(0),
            num_mip_levels: self.num_mip_levels.unwrap_or(0),
            num_samples: self.num_samples.unwrap_or(0),
            buffer: self.buffer.unwrap_or(null_mut()),
        }
    }
}

impl Into<cl_image_desc> for &ClImageDesc {
    fn into(self) -> cl_image_desc {
        cl_image_desc {
            image_type: self.image_type.into(),
            image_width: self.image_height.unwrap_or(0),
            image_height: self.image_height.unwrap_or(0),
            image_depth:  self.image_depth.unwrap_or(0),
            image_array_size: self.image_array_size.unwrap_or(0),
            image_row_pitch: self.image_row_pitch.unwrap_or(0),
            image_slice_pitch: self.image_slice_pitch.unwrap_or(0),
            num_mip_levels: self.num_mip_levels.unwrap_or(0),
            num_samples: self.num_samples.unwrap_or(0),
            buffer: self.buffer.unwrap_or(null_mut()),
        }
    }
}

