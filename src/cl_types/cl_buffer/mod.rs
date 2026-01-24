use std::os::raw::c_void;

use crate::{
    cl_types::{memory_flags::MemoryFlags, cl_context::ClContext, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
};

/// # ClBuffer
/// 
/// Represents a contiguous block of memory on the GPU (like an array or vector).
/// It's the most common resource for passing data to a kernel.
pub struct ClBuffer {
    value: *mut c_void,
}

impl ClBuffer {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value.clone()
    }

    /// Creates a new memory buffer in the specified context.
    /// 
    /// - `flags`: Permissions (e.g. ReadOnly, WriteOnly, CopyHostPtr).
    /// - `buffer_size`: Total size in bytes.
    /// - `host_ptr`: Optional pointer to CPU data to initialize the buffer.
    #[cfg(feature = "CL_VERSION_1_1")]
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

#[cfg(feature = "CL_VERSION_1_1")]
impl Clone for ClBuffer {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }
        Self { value: self.value }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClBuffer{
    fn drop(&mut self) {
        unsafe {
            cl3::memory::release_mem_object(self.value);
        };
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClBuffer {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::memory::retain_mem_object(self.as_ptr());
        }
    }
}

unsafe impl Sync for ClBuffer {}
unsafe impl Send for ClBuffer {}