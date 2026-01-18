use core::fmt;
use std::{os::raw::c_void, ptr::null};

#[cfg(feature = "CL_VERSION_1_1")]
use crate::{cl_types::cl_device::ClDevice, error::ClError};
use crate::{cl_context_generate_getters, cl_types::{cl_platform::ClPlatform, releaseable::Releaseable}};
use std::ptr::null_mut;
use crate::error::api_error::ApiError;

/// # ClContext
/// 
/// Represents an OpenCL context - the environment where kernels execute.
/// 
/// A context manages all the resources (buffers, images, programs, kernels) and
/// coordinates execution across one or more devices. Think of it as the "workspace"
/// where all your OpenCL operations happen.
#[derive(Debug)]
pub struct ClContext {
    value: *mut c_void,
}

impl ClContext {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    } 

    /// Creates a new context for the specified devices.
    /// 
    /// All devices must be from the same platform. The context will manage resources
    /// and coordinate execution across all these devices.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new(device_list: &Vec<ClDevice>) -> Result<Self, ClError> {
        

        let device_raw_ids: Vec<*mut c_void> = device_list
            .iter()
            .map(|device| device.as_ptr())
            .collect();
        let raw_context =
            cl3::context::create_context(&device_raw_ids, null_mut(), None, null_mut())
                .map_err(|e| ClError::Api(ApiError::get_error(e)))?;
        Ok(Self::from_ptr(raw_context))
    }

    /// Creates a context from all devices of a specific type on a platform.
    /// 
    /// This is a convenience method when you want to use all GPUs, all CPUs, etc.
    /// from a specific platform without manually querying devices.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new_from_device_type(
        platform: &ClPlatform,
        device_type: u64,
    ) -> Result<Self, ClError> {
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
        .map_err(|err| ClError::Api(ApiError::get_error(err)))?;
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

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClContext {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::context::retain_context(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClContext {
    fn drop(&mut self) {
        unsafe {
            cl3::context::release_context(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl std::fmt::Display for ClContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Context Id: {}, Num Devices: {}}}", self.value as isize, self.get_num_devices().unwrap_or_default())
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
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