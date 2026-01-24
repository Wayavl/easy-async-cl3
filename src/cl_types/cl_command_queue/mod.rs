pub mod command_queue_parameters;
use std::os::raw::c_void;
use std::ptr::null;

use crate::cl_types::cl_buffer::ClBuffer;
use crate::cl_types::cl_image::ClImage;
use crate::error::ClError;
use crate::error::api_error::ApiError;
use crate::{
    cl_command_queue_generate_getters,
    cl_types::{
        cl_context::ClContext, cl_device::ClDevice, cl_event::ClEvent, cl_kernel::ClKernel,
        releaseable::Releaseable,
    },
};
use std::future::Future;

#[derive(Copy, Clone)]
struct SendPtr(*mut c_void);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

pub struct ClCommandQueue {
    value: *mut c_void,
}

impl ClCommandQueue {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(pointer: *mut c_void) -> Self {
        Self { value: pointer }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    /// Creates a command queue (deprecated in OpenCL 2.0+).
    ///
    /// Use `create_command_queue_with_properties` for OpenCL 2.0+ for more flexibility.
    #[cfg(feature = "CL_VERSION_1_1")]
    #[deprecated(
        since = "CL_VERSION_2_0",
        note = "Use create_command_queue_with_properties instead"
    )]
    #[allow(deprecated)]
    pub fn create_command_queue(
        context: &ClContext,
        device: &ClDevice,
        properties: u64,
    ) -> Result<Self, ClError> {
        let raw_command_queue = unsafe {
            cl3::command_queue::create_command_queue(context.as_ptr(), device.as_ptr(), properties)
                .map_err(|data| ClError::Api(ApiError::get_error(data)))
        }?;

        Ok(Self {
            value: raw_command_queue,
        })
    }

    /// Creates a command queue with properties (OpenCL 2.0+).
    ///
    /// Allows specifying properties like out-of-order execution, profiling, etc.
    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn create_command_queue_with_properties(
        context: &ClContext,
        device: &ClDevice,
        properties: &Vec<u64>,
    ) -> Result<Self, ClError> {
        let raw_command_queue = unsafe {
            cl3::command_queue::create_command_queue_with_properties(
                context.as_ptr(),
                device.as_ptr(),
                properties.as_ptr(),
            )
            .map_err(|code| ClError::Api(ApiError::get_error(code)))
        }?;

        Ok(Self {
            value: raw_command_queue,
        })
    }

    /// Enqueues an N-dimensional kernel execution.
    ///
    /// This is the main method for executing kernels on the GPU. Returns a future
    /// that completes when the kernel finishes.
    #[cfg(feature = "CL_VERSION_1_1")]
    fn enqueue_nd_range_kernel_inner(
        q_ptr: SendPtr,
        k_ptr: SendPtr,
        work_dimension: u32,
        global_work_offset: Vec<usize>,
        global_work_dims: Vec<usize>,
        local_work_dims: Vec<usize>,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, std::ptr::null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            let global_offset_ptr = if global_work_offset.is_empty() {
                std::ptr::null()
            } else {
                global_work_offset.as_ptr()
            };

            let local_dims_ptr = if local_work_dims.is_empty() {
                std::ptr::null()
            } else {
                local_work_dims.as_ptr()
            };

            unsafe {
                cl3::command_queue::enqueue_nd_range_kernel(
                    q_ptr.0,
                    k_ptr.0,
                    work_dimension,
                    global_offset_ptr,
                    global_work_dims.as_ptr(),
                    local_dims_ptr,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn enqueue_nd_range_kernel(
        &self,
        kernel: &ClKernel,
        work_dimension: u32,
        global_work_offset: Vec<usize>,
        global_work_dims: Vec<usize>,
        local_work_dims: Vec<usize>,
        _num_events_in_wait_list: Option<u32>,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let k_ptr = SendPtr(kernel.as_ptr());

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::enqueue_nd_range_kernel_inner(
                q_ptr,
                k_ptr,
                work_dimension,
                global_work_offset,
                global_work_dims,
                local_work_dims,
                wait_ptrs,
            )?;
            event.event_future().await;
            Ok(event)
        }
    }

    /// Reads data from a buffer on the device to host memory.
    ///
    /// This is an async operation that completes when the data transfer finishes.
    #[cfg(feature = "CL_VERSION_1_1")]
    fn enqueue_read_buffer_inner(
        q_ptr: SendPtr,
        b_ptr: SendPtr,
        offset: usize,
        len: usize,
        h_ptr: SendPtr,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            unsafe {
                cl3::command_queue::enqueue_read_buffer(
                    q_ptr.0,
                    b_ptr.0,
                    0, // CL_FALSE
                    offset,
                    len,
                    h_ptr.0,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn enqueue_read_buffer<T: Sized>(
        &self,
        buffer: &ClBuffer,
        offset: Option<usize>,
        host_memory: &mut [T],
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let offset = offset.unwrap_or(0);
        let q_ptr = SendPtr(self.as_ptr());
        let b_ptr = SendPtr(buffer.as_ptr());
        let h_ptr = SendPtr(host_memory.as_mut_ptr() as *mut c_void);
        let len = host_memory.len();

        // Convert wait list to Send-safe SendPtrs
        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::enqueue_read_buffer_inner(q_ptr, b_ptr, offset, len, h_ptr, wait_ptrs)?;
            event.event_future().await;
            Ok(event)
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    fn enqueue_read_buffer_raw_inner(
        q_ptr: SendPtr,
        b_ptr: SendPtr,
        offset: usize,
        size: usize,
        h_ptr: SendPtr,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            unsafe {
                cl3::command_queue::enqueue_read_buffer(
                    q_ptr.0,
                    b_ptr.0,
                    0, // CL_FALSE
                    offset,
                    size,
                    h_ptr.0,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn enqueue_read_buffer_raw(
        &self,
        buffer: &ClBuffer,
        offset: Option<usize>,
        host_ptr: *mut c_void,
        size: usize,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let offset = offset.unwrap_or(0);
        let q_ptr = SendPtr(self.as_ptr());
        let b_ptr = SendPtr(buffer.as_ptr());
        let h_ptr = SendPtr(host_ptr);

        // Convert wait list to Send-safe SendPtrs
        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::enqueue_read_buffer_raw_inner(q_ptr, b_ptr, offset, size, h_ptr, wait_ptrs)?;
            event.event_future().await;
            Ok(event)
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    fn read_image_raw_inner(
        q_ptr: SendPtr,
        i_ptr: SendPtr,
        origin: [usize; 3],
        region: [usize; 3],
        row_pitch: usize,
        slice_pitch: usize,
        b_ptr: SendPtr,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            unsafe {
                cl3::command_queue::enqueue_read_image(
                    q_ptr.0,
                    i_ptr.0,
                    0, // CL_FALSE
                    origin.as_ptr(),
                    region.as_ptr(),
                    row_pitch,
                    slice_pitch,
                    b_ptr.0,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn read_image_raw(
        &self,
        image: &ClImage,
        origin: [usize; 3],
        region: [usize; 3],
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let i_ptr = SendPtr(image.as_ptr());
        let b_ptr = SendPtr(buffer);

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::read_image_raw_inner(
                q_ptr,
                i_ptr,
                origin,
                region,
                row_pitch,
                slice_pitch,
                b_ptr,
                wait_ptrs,
            )?;
            event.event_future().await;
            Ok(event)
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn read_image(
        &self,
        image: ClImage,
        origin: Vec<usize>,
        region: Vec<usize>,
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let i_ptr = SendPtr(image.as_ptr());
        let b_ptr = SendPtr(buffer);

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::read_image_raw_inner(
                q_ptr,
                i_ptr,
                origin.as_slice().try_into().unwrap_or([0, 0, 0]),
                region.as_slice().try_into().unwrap_or([0, 0, 0]),
                row_pitch,
                slice_pitch,
                b_ptr,
                wait_ptrs,
            )?;
            event.event_future().await;
            Ok(event)
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    fn write_image_raw_inner(
        q_ptr: SendPtr,
        i_ptr: SendPtr,
        origin: [usize; 3],
        region: [usize; 3],
        row_pitch: usize,
        slice_pitch: usize,
        b_ptr: SendPtr,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            unsafe {
                cl3::command_queue::enqueue_write_image(
                    q_ptr.0,
                    i_ptr.0,
                    0, // CL_FALSE
                    origin.as_ptr(),
                    region.as_ptr(),
                    row_pitch,
                    slice_pitch,
                    b_ptr.0,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn write_image_raw(
        &self,
        image: &ClImage,
        origin: [usize; 3],
        region: [usize; 3],
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let i_ptr = SendPtr(image.as_ptr());
        let b_ptr = SendPtr(buffer);

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::write_image_raw_inner(
                q_ptr,
                i_ptr,
                origin,
                region,
                row_pitch,
                slice_pitch,
                b_ptr,
                wait_ptrs,
            )?;
            event.event_future().await;
            Ok(event)
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn write_image(
        &self,
        image: ClImage,
        origin: Vec<usize>,
        region: Vec<usize>,
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let i_ptr = SendPtr(image.as_ptr());
        let b_ptr = SendPtr(buffer);

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::write_image_raw_inner(
                q_ptr,
                i_ptr,
                origin.as_slice().try_into().unwrap_or([0, 0, 0]),
                region.as_slice().try_into().unwrap_or([0, 0, 0]),
                row_pitch,
                slice_pitch,
                b_ptr,
                wait_ptrs,
            )?;
            event.event_future().await;
            Ok(event)
        }
    }

    fn write_buffer_inner(
        q_ptr: SendPtr,
        b_ptr: SendPtr,
        offset: usize,
        byte_size: usize,
        h_ptr: SendPtr,
        wait_ptrs: Option<Vec<SendPtr>>,
    ) -> Result<ClEvent, ClError> {
        let res = {
            let wait_raw: Vec<*mut c_void> = wait_ptrs
                .map(|v| v.iter().map(|p| p.0).collect())
                .unwrap_or_default();

            let (num_wait, wait_ptr) = if wait_raw.is_empty() {
                (0, null())
            } else {
                (wait_raw.len() as u32, wait_raw.as_ptr())
            };

            unsafe {
                cl3::command_queue::enqueue_write_buffer(
                    q_ptr.0,
                    b_ptr.0,
                    0, // CL_FALSE (Non-blocking)
                    offset,
                    byte_size,
                    h_ptr.0,
                    num_wait,
                    wait_ptr,
                )
            }
        };

        Ok(ClEvent::from_ptr(
            res.map_err(|code| ClError::Api(ApiError::get_error(code)))?,
        ))
    }

    pub fn write_buffer(
        &self,
        buffer: &ClBuffer,
        host_ptr: *mut c_void,
        offset: usize,
        byte_size: usize,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> impl Future<Output = Result<ClEvent, ClError>> + Send + '_ {
        let q_ptr = SendPtr(self.as_ptr());
        let b_ptr = SendPtr(buffer.as_ptr());
        let h_ptr = SendPtr(host_ptr);

        let wait_ptrs: Option<Vec<SendPtr>> =
            event_wait_list.map(|v| v.iter().map(|e| SendPtr(e.as_ptr())).collect());

        async move {
            let event = Self::write_buffer_inner(q_ptr, b_ptr, offset, byte_size, h_ptr, wait_ptrs)?;
            event.event_future().await;
            Ok(event)
        }
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

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClCommandQueue {
    fn drop(&mut self) {
        unsafe {
            cl3::command_queue::release_command_queue(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClCommandQueue {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::command_queue::retain_command_queue(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
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

unsafe impl Sync for ClCommandQueue {}
unsafe impl Send for ClCommandQueue {}
