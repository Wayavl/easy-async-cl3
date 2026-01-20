use std::os::raw::c_void;

use cl3::ext::{cl_image_desc, cl_image_format};

use crate::{
    cl_types::{
        memory_flags::MemoryFlags,
        cl_context::ClContext,
        cl_image::{
            image_channel_data_type::ClImageChannelType, image_channel_order::ClImageChannelOrder, image_desc::ClImageDesc, image_formats::ClImageFormats, image_type::ClImageType
        },
        cl_platform::ClPlatform,
        releaseable::Releaseable,
    },
    error::{ClError, api_error::ApiError},
    cl_image_generate_getters,
};

pub mod image_channel_data_type;
pub mod image_channel_order;
pub mod image_formats;
pub mod image_type;
pub mod image_desc;

/// # ClImage
/// 
/// Represents a texture or image on the GPU.
/// Unlike Buffers, images are optimized for spatial access (2D/3D)
/// and can use hardware filtering.
pub struct ClImage {
    value: *mut c_void
}

impl ClImage {
    /// Gets a list of image formats (like RGBA8, Float, etc.) 
    /// that your current graphics card supports.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_supported_image_formats(
        context: &ClContext,
        flags: &Vec<MemoryFlags>,
        image_type: ClImageType,
    ) -> Result<Vec<ClImageFormats>, ClError> {
        let array: Vec<cl3::ext::cl_image_format> = cl3::memory::get_supported_image_formats(
            context.as_ptr(),
            MemoryFlags::to_u64(&flags),
            image_type.into(),
        )
        .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(array
            .iter()
            .map(|f| ClImageFormats {
                image_channel_data_type: ClImageChannelType::from(f.image_channel_data_type),
                image_channel_order: ClImageChannelOrder::from(f.image_channel_order),
            })
            .collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value.clone()
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn new(context: &ClContext, flags: &Vec<MemoryFlags>, image_format: &ClImageFormats, image_desc: &ClImageDesc, host_ptr: *mut c_void ) -> Result<Self, ClError> {

        let image_format = cl_image_format {
            image_channel_data_type: image_format.image_channel_data_type.into(),
            image_channel_order: image_format.image_channel_order.into()
        };

        let desc: cl_image_desc = image_desc.into();

        let raw = unsafe {
            cl3::memory::create_image(context.as_ptr(), MemoryFlags::to_u64(flags), &image_format, &desc, host_ptr).map_err(|code| ClError::Api(ApiError::get_error(code)))?
        };
        
        Ok(Self {
            value: raw
        })
    }

    #[cfg(feature = "CL_VERSION_3_0")]
    pub fn new_with_properties(context: &ClContext, properties: Vec<u64> ,flags: &Vec<MemoryFlags>, image_format: &ClImageFormats, image_desc: &ClImageDesc, host_ptr: *mut c_void ) -> Result<Self, ClError> {
        let image_format = cl_image_format {
            image_channel_data_type: image_format.image_channel_data_type.into(),
            image_channel_order: image_format.image_channel_order.into()
        };

        let desc: cl_image_desc = image_desc.into();

        let raw = unsafe {
            cl3::memory::create_image_with_properties(context.as_ptr(), properties.as_ptr(),MemoryFlags::to_u64(flags), &image_format, &desc, host_ptr).map_err(|code| ClError::Api(ApiError::get_error(code)))?
        };
        
        Ok(Self {
            value: raw
        })
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_image_generate_getters!(
        (get_image_format, ClImageFormats, cl3::memory::CL_IMAGE_FORMAT),
        (get_element_size, usize, cl3::memory::CL_IMAGE_ELEMENT_SIZE),
        (get_row_pitch, usize, cl3::memory::CL_IMAGE_ROW_PITCH),
        (get_slice_pitch, usize, cl3::memory::CL_IMAGE_SLICE_PITCH),
        (get_width, usize, cl3::memory::CL_IMAGE_WIDTH),
        (get_height, usize, cl3::memory::CL_IMAGE_HEIGHT),
        (get_depth, usize, cl3::memory::CL_IMAGE_DEPTH),
        (get_array_size, usize, cl3::memory::CL_IMAGE_ARRAY_SIZE),
        (get_num_mip_levels, u32, cl3::memory::CL_IMAGE_NUM_MIP_LEVELS),
        (get_num_samples, u32, cl3::memory::CL_IMAGE_NUM_SAMPLES),
        (get_reference_count, u32, cl3::memory::CL_MEM_REFERENCE_COUNT),
    );
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClImage {
    fn drop(&mut self) {
        unsafe {
            cl3::memory::release_mem_object(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Clone for ClImage {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value,
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClImage {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::memory::retain_mem_object(self.value);
        }
    }
}


#[test]
fn supp() -> Result<(), ClError> {
    let platforms = ClPlatform::get_all()?;
    let platform = platforms.get(1).unwrap();
    print!("Platform: {}", platform);
    let devices = platform.get_all_devices()?;
    let context = ClContext::new(&devices)?;
    let device_support = devices.first().unwrap().clone();
    println!("Image support: {}", device_support.get_image_support()?);
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image1D)?
    );
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image1DArray)?
    );
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image1DBuffer)?
    );
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image2D)?
    );
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image2DArray)?
    );
    println!(
        "{:?}\n\n\n\n\n\n",
        ClImage::get_supported_image_formats(&context, &vec![MemoryFlags::ReadWrite], ClImageType::Image3D)?
    );
    Ok(())
}

unsafe impl Sync for ClImage {}
unsafe impl Send for ClImage {}