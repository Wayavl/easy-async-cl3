mod program_parameters;
use crate::{
    cl_types::{cl_context::ClContext, releaseable::Releaseable},
    error::{ClError, api_error::ApiError},
};
use std::{ffi::CStr, marker::PhantomData, os::raw::c_void, vec};

pub struct Builded;
pub struct NotBuilded;

pub struct ClProgram<T> {
    value: *mut c_void,
    phantom_value: PhantomData<T>,
}

impl<T> ClProgram<T> {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self {
            value,
            phantom_value: PhantomData,
        }
    }
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }
    pub fn from_src(context: &ClContext, source: String) -> Result<ClProgram<NotBuilded>, ClError> {
        let source: Vec<&str> = vec![source.as_str()];
        let raw_program =
            cl3::program::create_program_with_source(context.as_ptr(), source.as_slice())
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(ClProgram {
            value: raw_program,
            phantom_value: PhantomData,
        })
    }

    // pub fn from_binary(context: &ClContext, binary: Vec<u8>) {

    // }

    // pub fn from_il(
    //     context: &ClContext,
    //     il: Vec<u8>,
    // ) -> Result<ClProgram<NotBuilded>, ClError> {
    //     let raw_program = cl3::program::create_program_with_il(context.as_ptr(), il.as_slice())
    //         .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
    //     Ok(ClProgram {
    //         value: raw_program,
    //         phantom_value: PhantomData,
    //     })
    // }
}

impl<T> Clone for ClProgram<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value.clone(),
            phantom_value: PhantomData,
        }
    }
}

impl<T> Releaseable for ClProgram<T> {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::program::retain_program(self.value);
        }
    }
}

impl<T> Drop for ClProgram<T> {
    fn drop(&mut self) {
        unsafe {
            cl3::program::release_program(self.value);
        }
    }
}
