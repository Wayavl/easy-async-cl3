use std::{os::raw::c_void, ptr::null};

#[cfg(feature = "CL_VERSION_1_1")]
use crate::cl_types::cl_device::ClDevice;
use crate::{cl_types::cl_platform::ClPlatform, error::cl_context::ContextError};
use std::ptr::null_mut;
pub struct ClContext {
    value: cl3::types::cl_context,
}

impl ClContext {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new(device_list: &Vec<ClDevice>) -> Result<Self, ContextError> {
        let device_raw_ids: Vec<*mut c_void> = device_list
            .iter()
            .map(|device| device.get_device_id())
            .collect();
        let raw_context =
            cl3::context::create_context(&device_raw_ids, null_mut(), None, null_mut())
                .map_err(|e| ContextError::ErrorCreatingContext(e))?;
        Ok(Self::from_ptr(raw_context))
    }

    pub fn new_from_device_type(
        platform: &ClPlatform,
        device_type: u64,
    ) -> Result<Self, ContextError> {
        let properties = vec![
            cl3::context::CL_CONTEXT_PLATFORM,
            platform.get_platform_id() as isize,
            0,
        ];
        let raw_context = cl3::context::create_context_from_type(
            device_type,
            properties.as_ptr(),
            None,
            null_mut(),
        )
        .map_err(|err| ContextError::ErrorCreatingContext(err))?;
        Ok(ClContext::from_ptr(raw_context))
    }
}

impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe {
            let _ = cl3::context::release_context(self.value);
        }
    }
}
