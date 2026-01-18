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
    pub async fn enqueue_nd_range_kernel(
        &self,
        kernel: &ClKernel,
        work_dimension: u32,
        global_work_offset: Vec<usize>,
        global_work_dims: Vec<usize>,
        local_work_dims: Vec<usize>,
        num_events_in_wait_list: Option<u32>,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let event_wait_list_wrapped = event_wait_list.unwrap_or(Vec::new());
        let num_events_in_wait_list = event_wait_list_wrapped.len() as u32;
        let event_wait_list_ptr = if num_events_in_wait_list > 0 {
            event_wait_list_wrapped
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let event = ClEvent::from_ptr(unsafe {
            cl3::command_queue::enqueue_nd_range_kernel(
                self.as_ptr(),
                kernel.as_ptr(),
                work_dimension,
                if global_work_offset.is_empty() { std::ptr::null() } else { global_work_offset.as_ptr() },
                global_work_dims.as_ptr(),
                if local_work_dims.is_empty() { std::ptr::null() } else { local_work_dims.as_ptr() },
                num_events_in_wait_list,
                if num_events_in_wait_list > 0 {
                    event_wait_list_ptr.as_ptr()
                } else {
                    std::ptr::null()
                },
            )
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?
        });

        event.event_future().await;

        Ok(event)
    }

    /// Reads data from a buffer on the device to host memory.
    /// 
    /// This is an async operation that completes when the data transfer finishes.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub async fn enqueue_read_buffer<T: Sized>(
        &self,
        buffer: &ClBuffer,
        offset: Option<usize>,
        host_memory: &mut [T],
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let offset = offset.unwrap_or(0);
        let event_wait_list_wrapped = event_wait_list.unwrap_or(Vec::new());
        let event_wait_list: Vec<*mut c_void> =
            event_wait_list_wrapped.iter().map(|f| f.as_ptr()).collect();
        let num_events_in_wait_list = event_wait_list.len() as u32;

        let event = match num_events_in_wait_list {
            0 => ClEvent::from_ptr(
                unsafe {
                    cl3::command_queue::enqueue_read_buffer(
                        self.as_ptr(),
                        buffer.as_ptr(),
                        0,
                        offset,
                        host_memory.len(),
                        host_memory.as_mut_ptr() as *mut c_void,
                        num_events_in_wait_list,
                        null(),
                    )
                }
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?,
            ),
            _ => ClEvent::from_ptr(
                unsafe {
                    cl3::command_queue::enqueue_read_buffer(
                        self.as_ptr(),
                        buffer.as_ptr(),
                        0,
                        offset,
                        host_memory.len(),
                        host_memory.as_mut_ptr() as *mut c_void,
                        num_events_in_wait_list,
                        event_wait_list.as_ptr(),
                    )
                }
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?,
            ),
        };

        event.event_future().await;
        Ok(event)
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub async fn enqueue_read_buffer_raw(
        &self,
        buffer: &ClBuffer,
        offset: Option<usize>,
        host_ptr: *mut c_void,
        size: usize,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let offset = offset.unwrap_or(0);
        let event_wait_list_wrapped = event_wait_list.unwrap_or(Vec::new());
        let event_wait_list: Vec<*mut c_void> =
            event_wait_list_wrapped.iter().map(|f| f.as_ptr()).collect();
        let num_events_in_wait_list = event_wait_list.len() as u32;

        let event = match num_events_in_wait_list {
            0 => ClEvent::from_ptr(
                unsafe {
                    cl3::command_queue::enqueue_read_buffer(
                        self.as_ptr(),
                        buffer.as_ptr(),
                        0,
                        offset,
                        size,
                        host_ptr,
                        num_events_in_wait_list,
                        null(),
                    )
                }
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?,
            ),
            _ => ClEvent::from_ptr(
                unsafe {
                    cl3::command_queue::enqueue_read_buffer(
                        self.as_ptr(),
                        buffer.as_ptr(),
                        0,
                        offset,
                        size,
                        host_ptr,
                        num_events_in_wait_list,
                        event_wait_list.as_ptr(),
                    )
                }
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?,
            ),
        };

        event.event_future().await;
        Ok(event)
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub async fn read_image_raw(
        &self,
        image: &ClImage,
        origin: [usize; 3],
        region: [usize; 3],
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let raw = unsafe {
            match event_wait_list {
                Some(v) => {
                    let unwrapped_events: Vec<*mut c_void> = v.iter().map(|f| f.as_ptr()).collect();
                    cl3::command_queue::enqueue_read_image(
                        self.as_ptr(),
                        image.as_ptr(),
                        0,
                        origin.as_ptr(),
                        region.as_ptr(),
                        row_pitch,
                        slice_pitch,
                        buffer,
                        unwrapped_events.len() as u32,
                        unwrapped_events.as_ptr(),
                    )
                }
                None => cl3::command_queue::enqueue_read_image(
                    self.as_ptr(),
                    image.as_ptr(),
                    0,
                    origin.as_ptr(),
                    region.as_ptr(),
                    row_pitch,
                    slice_pitch,
                    buffer,
                    0,
                    null(),
                ),
            }
        }
        .map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        let wrapped_event = ClEvent::from_ptr(raw);
        wrapped_event.event_future().await;

        Ok(wrapped_event)
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub async fn read_image(
        &self,
        image: ClImage,
        origin: Vec<usize>,
        region: Vec<usize>,
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let raw = unsafe {
            match event_wait_list {
                Some(v) => {
                    let unwrapped_events: Vec<*mut c_void> = v.iter().map(|f| f.as_ptr()).collect();
                    cl3::command_queue::enqueue_read_image(
                        self.as_ptr(),
                        image.as_ptr(),
                        0,
                        origin.as_ptr(),
                        region.as_ptr(),
                        row_pitch,
                        slice_pitch,
                        buffer,
                        unwrapped_events.len() as u32,
                        unwrapped_events.as_ptr(),
                    )
                }
                None => cl3::command_queue::enqueue_read_image(
                    self.as_ptr(),
                    image.as_ptr(),
                    0,
                    origin.as_ptr(),
                    region.as_ptr(),
                    row_pitch,
                    slice_pitch,
                    buffer,
                    0,
                    null(),
                ),
            }
        }
        .map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        let wrapped_event = ClEvent::from_ptr(raw);
        wrapped_event.event_future().await;

        Ok(wrapped_event)
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub async fn write_image(
        &self,
        image: ClImage,
        origin: Vec<usize>,
        region: Vec<usize>,
        row_pitch: usize,
        slice_pitch: usize,
        buffer: *mut c_void,
        event_wait_list: Option<Vec<ClEvent>>,
    ) -> Result<ClEvent, ClError> {
        let raw = unsafe {
            match event_wait_list {
                Some(v) => {
                    let unwrapped_events: Vec<*mut c_void> = v.iter().map(|f| f.as_ptr()).collect();
                    cl3::command_queue::enqueue_write_image(
                        self.as_ptr(),
                        image.as_ptr(),
                        0,
                        origin.as_ptr(),
                        region.as_ptr(),
                        row_pitch,
                        slice_pitch,
                        buffer,
                        unwrapped_events.len() as u32,
                        unwrapped_events.as_ptr(),
                    )
                }
                None => cl3::command_queue::enqueue_write_image(
                    self.as_ptr(),
                    image.as_ptr(),
                    0,
                    origin.as_ptr(),
                    region.as_ptr(),
                    row_pitch,
                    slice_pitch,
                    buffer,
                    0,
                    null(),
                ),
            }
        }
        .map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        let wrapped_event = ClEvent::from_ptr(raw);
        wrapped_event.event_future().await;

        Ok(wrapped_event)
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
