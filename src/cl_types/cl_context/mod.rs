use core::fmt;
use std::{os::raw::c_void, ptr::null};

#[cfg(feature = "CL_VERSION_1_1")]
use crate::cl_types::cl_device::ClDevice;
use crate::{cl_context_generate_getters, cl_types::{cl_platform::ClPlatform, releaseable::Releaseable}, error::cl_context::ContextError};
use std::ptr::null_mut;

#[derive(Debug)]
pub struct ClContext {
    value: *mut c_void,
}

impl ClContext {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    } 

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new(device_list: &Vec<ClDevice>) -> Result<Self, ContextError> {
        let device_raw_ids: Vec<*mut c_void> = device_list
            .iter()
            .map(|device| device.as_ptr())
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
            platform.as_ptr() as isize,
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

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_context_generate_getters!(
        (get_context_reference_count, u32, cl3::context::CL_CONTEXT_REFERENCE_COUNT),
        (get_num_devices, u32, cl3::context::CL_CONTEXT_NUM_DEVICES),
        (get_devices, Vec<ClDevice>, cl3::context::CL_CONTEXT_DEVICES),
        (get_properties, Vec<isize>, cl3::context::CL_CONTEXT_PROPERTIES)
    );
}

impl Releaseable for ClContext {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::context::retain_context(self.value);
        }
    }
}

impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe {
            cl3::context::release_context(self.value);
        }
    }
}

impl std::fmt::Display for ClContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Context Id: {}, Num Devices: {}}}", self.value as isize, self.get_num_devices().unwrap_or_default())
    }
}

impl Clone for ClContext {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value.clone()
        }
    }
}