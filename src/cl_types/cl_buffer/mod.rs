use std::os::raw::c_void;

use crate::{
    cl_types::{buffer_flags::MemoryFlags, cl_context::ClContext, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
};

pub struct ClBuffer {
    value: *mut c_void,
}

impl ClBuffer {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value.clone()
    }

    pub fn new(
        context: &ClContext,
        flags: &Vec<MemoryFlags>,
        buffer_size: usize,
        host_ptr: *mut std::ffi::c_void,
    ) -> Result<Self, ClError> {
        let flags = MemoryFlags::to_u64(flags);
        let raw_ptr =
            unsafe { cl3::memory::create_buffer(context.as_ptr(), flags, buffer_size, host_ptr) }
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(Self { value: raw_ptr })
    }

    
}

impl Drop for ClBuffer{
    fn drop(&mut self) {
        unsafe {
            cl3::memory::retain_mem_object(self.as_ptr());
        }
    }
}

impl Releaseable for ClBuffer {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::memory::retain_mem_object(self.as_ptr());
        }
    }
}