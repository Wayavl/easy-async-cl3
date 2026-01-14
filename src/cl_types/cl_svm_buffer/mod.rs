use std::{marker::PhantomData, os::raw::c_void, ptr::null};

use cl3::context;

use crate::{cl_types::{memory_flags::MemoryFlags, cl_command_queue::ClCommandQueue, cl_context::ClContext}, error::{ClError, api_error::ApiError}};

pub struct ClSvmBuffer<T> {
    memory: *mut c_void,
    pub len: usize,
    context: ClContext,
    phantom: PhantomData<*mut T>,
}

impl<T> ClSvmBuffer<T> {
    pub fn new(context: &ClContext, flags: &Vec<MemoryFlags>, item_amount: usize, alignment: u32) -> Result<Self, ClError> {
        let raw_ptr = unsafe {
            cl3::memory::svm_alloc(context.as_ptr(), MemoryFlags::to_u64(&flags), item_amount * size_of::<T>(), alignment)
        }.map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        Ok(Self {
            memory: raw_ptr,
            len: item_amount,
            context: context.clone(),
            phantom: PhantomData
        })
    }

    pub fn map_mut<'a>(
        &'a mut self,
        queue: &'a ClCommandQueue,
        flags: &Vec<MemoryFlags>,
    ) -> Result<SvmMapGuard<'a, T>, ClError> {
        unsafe {
            cl3::command_queue::enqueue_svm_map(
                queue.as_ptr(),
                1,
                MemoryFlags::to_u64(&flags),
                self.memory,
                self.len * size_of::<T>(),
                0,
                null(),
            )
        }.map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        Ok(SvmMapGuard {
            ptr: self.memory as *mut T,
            len: self.len,
            queue,
            svm_ptr: self.memory,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *mut c_void { // Do not use to get access to the array.
        self.memory.clone()
    }
}

impl<T> Drop for ClSvmBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            cl3::memory::svm_free(self.context.as_ptr(), self.memory);
        }
    }
}

pub struct SvmMapGuard<'a, T> {
    ptr: *mut T,
    len: usize,
    queue: &'a ClCommandQueue,
    svm_ptr: *mut c_void,
    _marker: PhantomData<&'a mut [T]>,
}

impl<T> std::ops::Deref for SvmMapGuard<'_, T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl<T> std::ops::DerefMut for SvmMapGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }
}

impl<T> Drop for SvmMapGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let _ = cl3::command_queue::enqueue_svm_unmap(
                self.queue.as_ptr(),
                self.svm_ptr,
                0,
                null(),
            );
        }
    }
}
