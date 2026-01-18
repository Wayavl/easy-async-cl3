use std::os::raw::c_void;
use crate::{
    cl_types::{cl_context::ClContext, memory_flags::MemoryFlags, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
};

/// # ClPipe
/// 
/// Represents an OpenCL 2.0+ Pipe.
/// 
/// Pipes allow two kernels to communicate directly with each other (FIFO)
/// without having to pass data back to the CPU. It's like an internal streaming cable
/// within the GPU.
#[cfg(feature = "CL_VERSION_2_0")]
pub struct ClPipe {
    value: *mut c_void,
}

#[cfg(feature = "CL_VERSION_2_0")]
impl ClPipe {
    /// Creates a new Pipe.
    /// 
    /// - `pipe_packet_size`: The size of each data "packet" (e.g. 4 for an int).
    /// - `pipe_max_packets`: How many packets the pipe can hold at maximum.
    pub fn new(
        context: &ClContext,
        flags: &[MemoryFlags],
        pipe_packet_size: u32,
        pipe_max_packets: u32,
    ) -> Result<Self, ClError> {
        let raw_pipe = unsafe {
            cl3::memory::create_pipe(
                context.as_ptr(),
                MemoryFlags::to_u64(&flags.to_vec()),
                pipe_packet_size,
                pipe_max_packets,
            )
        }.map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        Ok(Self { value: raw_pipe })
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
impl Releaseable for ClPipe {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::memory::retain_mem_object(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
impl Drop for ClPipe {
    fn drop(&mut self) {
        unsafe {
            cl3::memory::release_mem_object(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_2_0")]
impl Clone for ClPipe {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }
        Self { value: self.value }
    }
}
