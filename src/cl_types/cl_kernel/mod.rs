use std::{ffi::CString, os::raw::c_void};

#[cfg(feature = "CL_VERSION_1_1")]
use crate::cl_types::cl_context::ClContext;
use crate::{
    cl_kernel_generate_getters, cl_kernel_workgroup_generate_getters, cl_types::{
        cl_program::{Builded, ClProgram},
        releaseable::Releaseable,
    }, error::{ClError, api_error::ApiError, wrapper_error::WrapperError}
};

/// # ClKernel
/// 
/// Represents an individual function that will execute on the GPU.
/// Kernels are extracted from an already compiled `ClProgram`. 
/// You can think of a Kernel as a single "work unit" or C function.
pub struct ClKernel {
    value: *mut c_void,
}

impl ClKernel {
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn from_ptr(value: *mut c_void) -> Self {
        Self { value }
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    /// Creates a new kernel by searching for a specific function in a compiled program.
    /// 
    /// - `program`: The program containing the code.
    /// - `kernel_name`: The exact name of the `kernel` function in the source code.
    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new(program: &ClProgram<Builded>, kernel_name: &str) -> Result<Self, ClError> {
        let cstr_kernel_name = CString::new(kernel_name)
            .map_err(|_| ClError::Wrapper(WrapperError::FailedToConvertStrToCString))?;
        let raw_kernel = cl3::kernel::create_kernel(program.as_ptr(), &cstr_kernel_name)
            .map_err(|err| ClError::Api(ApiError::get_error(err)))?;
        Ok(Self { value: raw_kernel })
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn new_in_program(program: ClProgram<Builded>) -> Result<Vec<Self>, ClError> {
        let mut raw_kernel = cl3::kernel::create_kernels_in_program(program.as_ptr())
            .map_err(|err| ClError::Api(ApiError::get_error(err)))?;
        let clkernels: Vec<Self> = raw_kernel
            .iter_mut()
            .map(|kernel| Self {
                value: kernel.clone(),
            })
            .collect();
        Ok(clkernels)
    }

    /// Creates an exact copy of this kernel (requires OpenCL 2.1+).
    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clone_kernel(&self) -> Result<Self, ClError> {
        let clone = cl3::kernel::clone_kernel(self.as_ptr()).map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(Self {
            value: clone
        })
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub unsafe fn set_args(&self, index: u32, byte_size: usize, value: *const c_void) -> Result<(), ClError> {
        unsafe {
            cl3::kernel::set_kernel_arg(
                self.value, index, byte_size, value
            );
        };

        Ok(())
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub unsafe fn set_svm_arg(&self, index: u32, byte_size: usize, value: *const c_void) -> Result<*const c_void, ClError> {

        unsafe {
            cl3::kernel::set_kernel_arg_svm_pointer(self.value, index, value)
        }.map_err(|code| ClError::Api(ApiError::get_error(code)))?;

        Ok(value)
    }

    // Todo Subgroup getters

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_kernel_generate_getters!(
        (get_function_name, String, cl3::kernel::CL_KERNEL_FUNCTION_NAME),
        (get_num_args, u32, cl3::kernel::CL_KERNEL_NUM_ARGS),
        (get_refence_count, u32, cl3::kernel::CL_KERNEL_REFERENCE_COUNT),
        (get_context, ClContext, cl3::kernel::CL_KERNEL_CONTEXT),
        (get_program, ClProgram<Builded>, cl3::kernel::CL_KERNEL_PROGRAM),
    );

    #[cfg(feature = "CL_VERSION_1_2")]
    cl_kernel_generate_getters!(
        (get_attributes, String, cl3::kernel::CL_KERNEL_ATTRIBUTES)
    );

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_kernel_workgroup_generate_getters!(
        (get_work_group_size, usize, cl3::kernel::CL_KERNEL_WORK_GROUP_SIZE),
        (get_compile_work_group_size, Vec<usize>, cl3::kernel::CL_KERNEL_WORK_GROUP_SIZE),
        (get_local_mem_size, u64, cl3::kernel::CL_KERNEL_LOCAL_MEM_SIZE),
        (get_preferred_work_group_size_multiple, usize, cl3::kernel::CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE),
        (get_private_mem_size, u64, cl3::kernel::CL_KERNEL_PRIVATE_MEM_SIZE)
    );

    #[cfg(feature = "CL_VERSION_1_2")]
    cl_kernel_workgroup_generate_getters!(
        (get_global_work_size, Vec<usize>, cl3::kernel::CL_KERNEL_GLOBAL_WORK_SIZE)
    );
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Drop for ClKernel {
    fn drop(&mut self) {
        unsafe {
            cl3::kernel::release_kernel(self.value);
        };
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Clone for ClKernel {
    fn clone(&self) -> Self {
        unsafe {
            cl3::kernel::retain_kernel(self.value);
        }
        Self { value: self.value }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl Releaseable for ClKernel {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::kernel::retain_kernel(self.value);
        }
    }
}



unsafe impl Sync for ClKernel {}
unsafe impl Send for ClKernel {}