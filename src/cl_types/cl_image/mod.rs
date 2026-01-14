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
    },
    error::{ClError, api_error::ApiError},
};

pub mod image_channel_data_type;
pub mod image_channel_order;
pub mod image_formats;
pub mod image_type;
pub mod image_desc;

pub struct ClImage {
    value: *mut c_void
}

impl ClImage {
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

    pub fn as_ptr(&self) -> *mut c_void {
        self.value.clone()
    }

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
