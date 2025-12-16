use std::{fmt, os::raw::c_void};
use crate::{error::cl_platform::PlatformError, generate_getters, cl_types::formater::Formater};

mod macros;
pub struct ClPlatform
{
    value: cl3::types::cl_platform_id
}

impl ClPlatform {
    pub fn new_from_c_void(value: *mut c_void) -> Self {
        ClPlatform { value }
    }

    pub fn new(value: cl3::types::cl_platform_id) -> Self {
        
        
        ClPlatform { value }
    }

    pub fn get_all() -> Result<Vec<Self>, PlatformError> {
        let mut raw_pointers = cl3::platform::get_platform_ids().map_err(PlatformError::GetIdError)?;
        Ok(
            raw_pointers.iter_mut().map(|p| Self::new_from_c_void(*p)).collect()
        )
    }

    pub fn default() -> Result<Self, PlatformError> {
        todo!("It will give the highest version platform available by default")
    }
    generate_getters!(
        (get_extensions, String, cl3::platform::CL_PLATFORM_EXTENSIONS),
        (get_extensions_with_version, String, cl3::platform::CL_PLATFORM_EXTENSIONS_WITH_VERSION),
        (get_external_memory_import_handle_types_khr, String, cl3::platform::CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR),
        (get_host_timer_resolution, String, cl3::platform::CL_PLATFORM_HOST_TIMER_RESOLUTION),
        (get_name, String, cl3::platform::CL_PLATFORM_NAME),
        (get_numeric_version, u32, cl3::platform::CL_PLATFORM_NUMERIC_VERSION),
        (get_profile, String, cl3::platform::CL_PLATFORM_PROFILE),
        (get_semaphore_export_handle_types_khr, String, cl3::platform::CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR),
        (get_semaphore_import_handle_types_khr, String, cl3::platform::CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR),
        (get_semaphore_types_khr, String, cl3::platform::CL_PLATFORM_SEMAPHORE_TYPES_KHR),
        (get_vendor, String, cl3::platform::CL_PLATFORM_VENDOR),
        (get_version, String, cl3::platform::CL_PLATFORM_VERSION),
    );
}

impl fmt::Display for ClPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VENDOR: {}, VERSION: {}, NUMERIC_VERSION: {}", self.get_vendor().unwrap_or_default(), self.get_version().unwrap_or_default(), self.get_numeric_version().unwrap_or_default())
    }
}