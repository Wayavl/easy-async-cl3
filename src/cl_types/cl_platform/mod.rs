use crate::cl_platform_generate_getters;
use crate::cl_types::cl_device::ClDevice;
#[cfg(feature = "CL_VERSION_1_1")]
use crate::error::ClError;
use crate::{cl_types::formatter::Formatter};
use std::{fmt, os::raw::c_void};
use crate::error::api_error::ApiError;

pub struct ClPlatform {
    value: *mut c_void,
}

impl ClPlatform {
    pub fn new(value: *mut c_void) -> Self {
        ClPlatform { value }
    }
    
    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_all() -> Result<Vec<Self>, ClError> {
        use crate::error::api_error::ApiError;

        let mut raw_pointers =
            cl3::platform::get_platform_ids().map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_pointers.iter_mut().map(|p| Self::new(*p)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn default() -> Result<Self, ClError> {
        let platforms = Self::get_all()?;
        let mut min_version = 0;
        let mut platform: Option<Self> = None;

        for p in platforms {
            let v = p.get_numeric_version().unwrap_or_default();
            if v >= min_version {
                min_version = v;
                platform = Some(p);
            }
        }
        
        platform.ok_or(ClError::Wrapper(crate::error::wrapper_error::WrapperError::DefaultPlatformNotFound))
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_platform_generate_getters!(
        (
            get_extensions,
            String,
            cl3::platform::CL_PLATFORM_EXTENSIONS
        ),
        (
            get_extensions_with_version,
            String,
            cl3::platform::CL_PLATFORM_EXTENSIONS_WITH_VERSION
        ),
        (
            get_external_memory_import_handle_types_khr,
            String,
            cl3::platform::CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR
        ),
        (get_name, String, cl3::platform::CL_PLATFORM_NAME),
        (
            get_numeric_version,
            u32,
            cl3::platform::CL_PLATFORM_NUMERIC_VERSION
        ),
        (get_profile, String, cl3::platform::CL_PLATFORM_PROFILE),
        (
            get_semaphore_export_handle_types_khr,
            String,
            cl3::platform::CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR
        ),
        (
            get_semaphore_import_handle_types_khr,
            String,
            cl3::platform::CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR
        ),
        (
            get_semaphore_types_khr,
            String,
            cl3::platform::CL_PLATFORM_SEMAPHORE_TYPES_KHR
        ),
        (get_vendor, String, cl3::platform::CL_PLATFORM_VENDOR),
        (get_version, String, cl3::platform::CL_PLATFORM_VERSION),
    );

    #[cfg(feature = "CL_VERSION_2_1")]
    cl_platform_generate_getters!((
        get_host_timer_resolution,
        String,
        cl3::platform::CL_PLATFORM_HOST_TIMER_RESOLUTION
    ),);

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_all_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices: Vec<*mut std::ffi::c_void> = cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_ALL)
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_gpu_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices = cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_GPU)
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_cpu_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices = cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_CPU)
            .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn get_custom_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices =
            cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_CUSTOM)
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_accelerator_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices =
            cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_ACCELERATOR)
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn get_default_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        let raw_devices =
            cl3::device::get_device_ids(self.value, cl3::device::CL_DEVICE_TYPE_DEFAULT)
                .map_err(|code| ClError::Api(ApiError::get_error(code)))?;
        Ok(raw_devices.iter().map(|dev| ClDevice::new(*dev)).collect())
    }

}

impl fmt::Display for ClPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VENDOR: {}, VERSION: {}, NUMERIC_VERSION: {}",
            self.get_vendor().unwrap_or_default(),
            self.get_version().unwrap_or_default(),
            self.get_numeric_version().unwrap_or_default()
        )
    }
}
