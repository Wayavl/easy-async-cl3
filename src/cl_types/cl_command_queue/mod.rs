pub mod command_queue_parameters;
use std::os::raw::c_void;

use crate::{
    cl_command_queue_generate_getters,
    cl_types::{cl_context::ClContext, cl_device::ClDevice, releaseable::Releaseable},
    error::cl_command_queue::CommandQueueError,
};

pub struct ClCommandQueue {
    value: *mut c_void,
}

impl ClCommandQueue {
    pub fn from_ptr(pointer: *mut c_void) -> Self {
        Self { value: pointer }
    }

    pub fn as_ptr(&self) -> *mut c_void{
        self.value
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    #[deprecated(
        since = "CL_VERSION_2_0",
        note = "Use create_command_queue_with_properties instead"
    )]

    pub fn create_command_queue(
        context: &ClContext,
        device: &ClDevice,
        properties: u64,
    ) -> Result<Self, CommandQueueError> {
        let raw_command_queue = unsafe {
            cl3::command_queue::create_command_queue(
                context.as_ptr(),
                device.as_ptr(),
                properties,
            )
            .map_err(CommandQueueError::CommandQueueFailed)
        }?;

        Ok(Self {
            value: raw_command_queue,
        })
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn create_command_queue_with_properties(
        context: &ClContext,
        device: &ClDevice,
        properties: &Vec<u64>,
    ) -> Result<Self, CommandQueueError> {
        let raw_command_queue = unsafe {
            cl3::command_queue::create_command_queue_with_properties(
                context.as_ptr(),
                device.as_ptr(),
                properties.as_ptr(),
            )
            .map_err(CommandQueueError::CommandQueueWithPropertiesFailed)
        }?;

        Ok(Self {
            value: raw_command_queue,
        })
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_command_queue_generate_getters!(
        (get_context, ClContext, cl3::command_queue::CL_QUEUE_CONTEXT),
        (get_device, ClDevice, cl3::command_queue::CL_QUEUE_DEVICE),
        (
            get_reference_count,
            u32,
            cl3::command_queue::CL_QUEUE_REFERENCE_COUNT
        ),
        (get_properties, u64, cl3::command_queue::CL_QUEUE_PROPERTIES),
    );

    #[cfg(feature = "CL_VERSION_2_0")]
    cl_command_queue_generate_getters!((get_queue_size, u32, cl3::command_queue::CL_QUEUE_SIZE));

    #[cfg(feature = "CL_VERSION_3_0")]
    cl_command_queue_generate_getters!((
        get_properties_array,
        Vec<u64>,
        cl3::command_queue::CL_QUEUE_PROPERTIES_ARRAY
    ));


}

impl Drop for ClCommandQueue {
    fn drop(&mut self) {
        unsafe {
            cl3::command_queue::release_command_queue(self.value);
        }
    }
}

impl Releaseable for ClCommandQueue {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::command_queue::retain_command_queue(self.value);
        }
    }
}

impl Clone for ClCommandQueue {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value.clone(),
        }
    }
}
