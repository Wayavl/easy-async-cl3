use std::os::raw::c_void;

use crate::cl_types::releaseable::Releaseable;

pub struct ClProgram {
    value: *mut c_void
}

impl ClProgram {
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self {
            value
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }
}

impl Clone for ClProgram {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value.clone()
        }
    }
}

impl Releaseable for ClProgram {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::program::retain_program(self.value);
        }
    }
}

impl Drop for ClProgram {
    fn drop(&mut self) {
        unsafe {
            cl3::program::release_program(self.value);
        }
    }
}