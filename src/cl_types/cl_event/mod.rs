use std::os::raw::c_void;

use cl3::gl::{cl_event, cl_int};
use tokio::sync::oneshot;

use crate::{
    cl_types::{cl_context::ClContext, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
    cl_event_profiling_generate_getters,
};

/// # ClEvent
/// 
/// Represents an OpenCL event - a synchronization primitive for tracking operations.
/// 
/// Events allow you to:
/// - Wait for GPU operations to complete
/// - Create dependencies between operations
/// - Profile execution times (when profiling is enabled)
/// - Coordinate asynchronous execution
pub struct ClEvent {
    value: *mut c_void,
}

impl ClEvent {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value: value }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    /// Creates a user event that can be manually signaled.
    /// 
    /// User events are useful for coordinating between host code and GPU operations.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new(context: &ClContext) -> Result<Self, ClError> {
        let raw_event = cl3::event::create_user_event(context.as_ptr())
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(Self { value: raw_event })
    }

    /// Creates a future that completes when this event completes.
    /// 
    /// This allows you to use async/await syntax to wait for GPU operations.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn event_future(&self) -> impl std::future::Future<Output = ()> {
        let (tx, rx) = oneshot::channel::<()>();

        extern "C" fn callback(
            _event: cl_event,
            _status: cl_int,
            user_data: *mut std::ffi::c_void,
        ) {
            let tx = unsafe { Box::from_raw(user_data as *mut oneshot::Sender<()>) };
            let _ = tx.send(());
        }

        let boxed_tx = Box::new(tx);

        unsafe {
            cl3::event::set_event_callback(
                self.value,
                cl3::event::CL_COMPLETE,
                callback,
                Box::into_raw(boxed_tx) as *mut _,
            );
        }

        async move {
            let _ = rx.await;
        }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_event_profiling_generate_getters!(
        (get_profiling_command_queued, u64, cl3::event::CL_PROFILING_COMMAND_QUEUED),
        (get_profiling_command_submit, u64, cl3::event::CL_PROFILING_COMMAND_SUBMIT),
        (get_profiling_command_start, u64, cl3::event::CL_PROFILING_COMMAND_START),
        (get_profiling_command_end, u64, cl3::event::CL_PROFILING_COMMAND_END),
    );

    /// Gets the execution duration in nanoseconds (requires profiling to be enabled).
    /// 
    /// Returns the time between when the command started and ended on the device.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_duration_nanos(&self) -> Result<u64, ClError> {
        let start = self.get_profiling_command_start()?;
        let end = self.get_profiling_command_end()?;
        if end >= start {
            Ok(end - start)
        } else {
            Ok(0)
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClEvent {
    fn drop(&mut self) {
        unsafe {
            cl3::event::release_event(self.as_ptr())
        };
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClEvent {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::event::retain_event(self.as_ptr());
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Clone for ClEvent {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }
        
        Self {
            value: self.value.clone()
        }
    }
}
