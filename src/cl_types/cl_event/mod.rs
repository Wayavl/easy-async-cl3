use std::os::raw::c_void;

use cl3::gl::{cl_event, cl_int};
use tokio::sync::oneshot;

use crate::{
    cl_types::{cl_context::ClContext, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
};

pub struct ClEvent {
    value: *mut c_void,
}

impl ClEvent {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value: value }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    pub fn new(context: &ClContext) -> Result<Self, ClError> {
        let raw_event = cl3::event::create_user_event(context.as_ptr())
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(Self { value: raw_event })
    }

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
}

impl Drop for ClEvent {
    fn drop(&mut self) {
        unsafe {
            cl3::event::release_event(self.as_ptr())
        };
    }
}

impl Releaseable for ClEvent {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::event::retain_event(self.as_ptr());
        }
    }
}

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
