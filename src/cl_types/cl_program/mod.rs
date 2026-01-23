pub mod program_parameters;
pub mod program_build_status;
pub mod program_binary_type;
use cl3::{ext::{CL_PROGRAM_BINARIES, CL_PROGRAM_BUILD_STATUS}, program::{build_program, get_program_build_data, create_program_with_binary}};
#[cfg(feature = "CL_VERSION_2_1")]
use cl3::program::create_program_with_il;

#[cfg(feature = "CL_VERSION_1_1")]
use crate::cl_types::cl_program::{program_binary_type::ProgramBinaryType, program_build_status::ProgramBuildStatus};
use crate::{
    cl_program_build_generate_getters, cl_program_generate_getters, cl_types::{cl_context::ClContext, cl_device::ClDevice, releaseable::Releaseable}, error::{ClError, api_error::ApiError, wrapper_error::WrapperError}
};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ops::Not,
    os::raw::c_void,
    ptr::null_mut,
    vec,
};

pub struct Builded;
pub struct NotBuilded;

pub struct ClProgram<T> {
    value: *mut c_void,
    phantom_value: PhantomData<T>,
}

impl<T> ClProgram<T> {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self {
            value,
            phantom_value: PhantomData,
        }
    }
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }
    /// Creates a program from OpenCL C source code.
    /// 
    /// The program is not yet compiled - you must call `build()` before creating kernels.
    #[cfg(feature = "CL_VERSION_1_1")]
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

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_program_generate_getters!(
        (get_context, ClContext, cl3::program::CL_PROGRAM_CONTEXT),
        (get_reference_count, u32, cl3::program::CL_PROGRAM_REFERENCE_COUNT),
        (get_num_devices, u32, cl3::program::CL_PROGRAM_NUM_DEVICES),
        (get_devices, Vec<ClDevice>, cl3::program::CL_PROGRAM_DEVICES),
        (get_source, String, cl3::program::CL_PROGRAM_SOURCE),
        (get_binary_sizes, Vec<isize>, cl3::program::CL_PROGRAM_BINARY_SIZES),
    );

    #[cfg(feature = "CL_VERSION_1_2")]
    cl_program_generate_getters!(
        (get_num_kernels, usize, cl3::program::CL_PROGRAM_NUM_KERNELS),
        (get_kernel_names, String, cl3::program::CL_PROGRAM_KERNEL_NAMES),
    );

    #[cfg(feature = "CL_VERSION_2_1")]
    cl_program_generate_getters!(
        (get_il, String, cl3::program::CL_PROGRAM_IL),
    );

    #[cfg(feature = "CL_VERSION_2_2")]
    cl_program_generate_getters!(
        (get_scope_global_ctors_present, u32, cl3::program::CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT),
        (get_scope_global_dtors_present, u32, cl3::program::CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT),
    );

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_program_build_generate_getters!(
        (get_build_status, ProgramBuildStatus, cl3::program::CL_PROGRAM_BUILD_STATUS),
        (get_build_options, String, cl3::program::CL_PROGRAM_BUILD_OPTIONS),
        (get_logs, String, cl3::program::CL_PROGRAM_BUILD_LOG),
    );

    #[cfg(feature = "CL_VERSION_1_1")]
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_binary(&self) -> Result<Vec<Vec<u8>>, ClError> {
        // Implement when cl3 supports get_program_binaries or exposes clGetProgramInfo
        Err(ClError::Wrapper(WrapperError::FormatterFailed)) 
    }

}

impl ClProgram<NotBuilded> {
    /// Builds (compiles) the program for the specified devices.
    /// 
    /// Returns a `ClProgram<Builded>` on success, which can be used to create kernels.
    /// Build options can include optimization flags, defines, etc.
    #[cfg(feature = "CL_VERSION_1_1")]
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_binary(
        context: &ClContext,
        devices: &[ClDevice],
        binaries: &[&[u8]],
    ) -> Result<ClProgram<NotBuilded>, ClError> {
        let device_ptrs: Vec<*mut c_void> = devices.iter().map(|d| d.as_ptr()).collect();
        let raw_program = unsafe {
             create_program_with_binary(
                context.as_ptr(),
                &device_ptrs,
                binaries,
            )
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?
        };

        Ok(ClProgram {
            value: raw_program,
            phantom_value: PhantomData,
        })
    }

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn from_il(
        context: &ClContext,
        il: &[u8],
    ) -> Result<ClProgram<NotBuilded>, ClError> {
        let raw_program = unsafe {
            create_program_with_il(context.as_ptr(), il)
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?
        };
        Ok(ClProgram {
            value: raw_program,
            phantom_value: PhantomData,
        })
    }

    /// Builds (compiles) the program for the specified devices.
    /// 
    /// Returns a `ClProgram<Builded>` on success, which can be used to create kernels.
    /// Build options can include optimization flags, defines, etc.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn build(
        &self,
        options: &String,
        devices: &Vec<ClDevice>,
    ) -> Result<ClProgram<Builded>, ClError> {
        let cstr_options = CString::new(options.as_str())
            .map_err(|_| ClError::Wrapper(WrapperError::FailedToConvertStrToCString))?;

        let devices_reference: Vec<*mut c_void> = devices.iter().map(|f| f.as_ptr()).collect();

        build_program(
            self.value,
            &devices_reference,
            cstr_options.as_c_str(),
            None,
            null_mut(),
        )
        .map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        unsafe {
            self.increase_reference_count();
        }

        Ok(ClProgram {
            value: self.value,
            phantom_value: PhantomData,
        })
    }
}

impl ClProgram<Builded> {

    

    #[cfg(feature = "CL_VERSION_1_2")]
    cl_program_build_generate_getters!(
        (get_binary_type, ProgramBinaryType, cl3::program::CL_PROGRAM_BINARY_TYPE)
    );

    #[cfg(feature = "CL_VERSION_2_0")]
    cl_program_build_generate_getters!(
        (get_build_global_variable_total_size, ProgramBinaryType, cl3::program::CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE)
    );

}

#[cfg(feature = "CL_VERSION_1_1")]
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

#[cfg(feature = "CL_VERSION_1_1")]
impl<T> Releaseable for ClProgram<T> {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::program::retain_program(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl<T> Drop for ClProgram<T> {
    fn drop(&mut self) {
        unsafe {
            cl3::program::release_program(self.value);
        }
    }
}


unsafe impl Sync for ClProgram<Builded> {}
unsafe impl Send for ClProgram<Builded> {}

unsafe impl Sync for ClProgram<NotBuilded> {}
unsafe impl Send for ClProgram<NotBuilded> {}