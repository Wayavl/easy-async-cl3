pub mod device_type;
use std::os::raw::c_void;

use crate::{
    cl_device_generate_getters, cl_types::releaseable::Releaseable, error::cl_device::DeviceError,
};

#[derive(Debug, Default)]
pub struct ClDevice {
    value: *mut c_void,
}

impl ClDevice {
    pub fn new(value: *mut c_void) -> Self {
        Self { value }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.value
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn create_subdevice_equally(&self, units: isize) -> Result<Vec<Self>, DeviceError> {
        if self.get_partition_max_sub_devices()? <= 0 {
            return Err(DeviceError::SubdeviceNotAvailable);
        }

        let properties = vec![
            cl3::device::CL_DEVICE_PARTITION_EQUALLY as cl3::device::cl_device_partition_property,
            units as cl3::device::cl_device_partition_property,
            0,
        ];
        let raw_subdevice_array = cl3::device::create_sub_devices(self.value, &properties)
            .map_err(DeviceError::CouldNotDivideDevice)?;

        Ok(raw_subdevice_array
            .iter()
            .map(|sub| ClDevice::new(*sub))
            .collect())
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn create_subdevice_by_count(
        &self,
        units: &mut Vec<isize>,
    ) -> Result<Vec<Self>, DeviceError> {
        if self.get_partition_max_sub_devices()? <= 0 {
            return Err(DeviceError::SubdeviceNotAvailable);
        }

        let mut properties = vec![cl3::device::CL_DEVICE_PARTITION_BY_COUNTS];
        properties.append(units);
        properties.push(cl3::device::CL_DEVICE_PARTITION_BY_COUNTS_LIST_END);

        let raw_subdevice_array = cl3::device::create_sub_devices(self.value, &properties)
            .map_err(DeviceError::CouldNotDivideDevice)?;
        Ok(raw_subdevice_array
            .iter()
            .map(|sub| {
                ClDevice::new(*sub)
            })
            .collect())
    }

    #[cfg(feature = "CL_VERSION_1_1")]
    cl_device_generate_getters!(
        (get_device_type, u64, cl3::device::CL_DEVICE_TYPE),
        (get_vendor_id, u32, cl3::device::CL_DEVICE_VENDOR_ID),
        (
            get_max_compute_units,
            u32,
            cl3::device::CL_DEVICE_MAX_COMPUTE_UNITS
        ),
        (
            get_max_work_item_dimensions,
            u32,
            cl3::device::CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS
        ),
        (
            get_max_work_item_sizes,
            Vec<usize>,
            cl3::device::CL_DEVICE_MAX_WORK_ITEM_SIZES
        ),
        (
            get_max_work_group_size,
            usize,
            cl3::device::CL_DEVICE_MAX_WORK_GROUP_SIZE
        ),
        (
            get_preferred_vector_width_char,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR
        ),
        (
            get_preferred_vector_width_short,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT
        ),
        (
            get_preferred_vector_width_int,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT
        ),
        (
            get_preferred_vector_width_long,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG
        ),
        (
            get_preferred_vector_width_float,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT
        ),
        (
            get_preferred_vector_width_double,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE
        ),
        (
            get_preferred_vector_width_half,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF
        ),
        (
            get_native_vector_width_char,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR
        ),
        (
            get_native_vector_width_short,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT
        ),
        (
            get_native_vector_width_int,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_INT
        ),
        (
            get_native_vector_width_long,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG
        ),
        (
            get_native_vector_width_float,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT
        ),
        (
            get_native_vector_width_double,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE
        ),
        (
            get_native_vector_width_half,
            u32,
            cl3::device::CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF
        ),
        (
            get_max_clock_frequency,
            u32,
            cl3::device::CL_DEVICE_MAX_CLOCK_FREQUENCY
        ),
        (get_address_bits, u32, cl3::device::CL_DEVICE_ADDRESS_BITS),
        (
            get_max_mem_alloc_size,
            u64,
            cl3::device::CL_DEVICE_MAX_MEM_ALLOC_SIZE
        ),
        (
            get_image_support,
            bool,
            cl3::device::CL_DEVICE_IMAGE_SUPPORT
        ),
        (
            get_max_read_image_args,
            u32,
            cl3::device::CL_DEVICE_MAX_READ_IMAGE_ARGS
        ),
        (
            get_max_write_image_args,
            u32,
            cl3::device::CL_DEVICE_MAX_WRITE_IMAGE_ARGS
        ),
        (
            get_image2d_max_width,
            usize,
            cl3::device::CL_DEVICE_IMAGE2D_MAX_WIDTH
        ),
        (
            get_image2d_max_height,
            usize,
            cl3::device::CL_DEVICE_IMAGE2D_MAX_HEIGHT
        ),
        (
            get_image3d_max_width,
            usize,
            cl3::device::CL_DEVICE_IMAGE3D_MAX_WIDTH
        ),
        (
            get_image3d_max_height,
            usize,
            cl3::device::CL_DEVICE_IMAGE3D_MAX_HEIGHT
        ),
        (
            get_image3d_max_depth,
            usize,
            cl3::device::CL_DEVICE_IMAGE3D_MAX_DEPTH
        ),
        (get_max_samplers, u32, cl3::device::CL_DEVICE_MAX_SAMPLERS),
        (
            get_max_parameter_size,
            usize,
            cl3::device::CL_DEVICE_MAX_PARAMETER_SIZE
        ),
        (
            get_mem_base_addr_align,
            u32,
            cl3::device::CL_DEVICE_MEM_BASE_ADDR_ALIGN
        ),
        (
            get_min_data_type_align_size,
            u32,
            cl3::device::CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE
        ),
        (
            get_single_fp_config,
            u32,
            cl3::device::CL_DEVICE_SINGLE_FP_CONFIG
        ),
        (
            get_global_mem_cache_type,
            u32,
            cl3::device::CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
        ),
        (
            get_global_mem_cacheline_size,
            u32,
            cl3::device::CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE
        ),
        (
            get_global_mem_cache_size,
            u64,
            cl3::device::CL_DEVICE_GLOBAL_MEM_CACHE_SIZE
        ),
        (
            get_global_mem_size,
            u64,
            cl3::device::CL_DEVICE_GLOBAL_MEM_SIZE
        ),
        (
            get_max_constant_buffer_size,
            u64,
            cl3::device::CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE
        ),
        (
            get_max_constant_args,
            u32,
            cl3::device::CL_DEVICE_MAX_CONSTANT_ARGS
        ),
        (
            get_local_mem_type,
            u32,
            cl3::device::CL_DEVICE_LOCAL_MEM_TYPE
        ),
        (
            get_local_mem_size,
            u64,
            cl3::device::CL_DEVICE_LOCAL_MEM_SIZE
        ),
        (
            get_error_correction_support,
            bool,
            cl3::device::CL_DEVICE_ERROR_CORRECTION_SUPPORT
        ),
        (
            get_host_unified_memory,
            bool,
            cl3::device::CL_DEVICE_HOST_UNIFIED_MEMORY
        ),
        (
            get_profiling_timer_resolution,
            usize,
            cl3::device::CL_DEVICE_PROFILING_TIMER_RESOLUTION
        ),
        (
            get_endian_little,
            bool,
            cl3::device::CL_DEVICE_ENDIAN_LITTLE
        ),
        (get_available, bool, cl3::device::CL_DEVICE_AVAILABLE),
        (
            get_compiler_available,
            bool,
            cl3::device::CL_DEVICE_COMPILER_AVAILABLE
        ),
        (
            get_execution_capabilities,
            u32,
            cl3::device::CL_DEVICE_EXECUTION_CAPABILITIES
        ),
        (get_name, String, cl3::device::CL_DEVICE_NAME),
        (get_vendor, String, cl3::device::CL_DEVICE_VENDOR),
        (get_driver_version, String, cl3::device::CL_DRIVER_VERSION),
        (get_profile, String, cl3::device::CL_DEVICE_PROFILE),
        (get_version, String, cl3::device::CL_DEVICE_VERSION),
        (
            get_opencl_c_version,
            String,
            cl3::device::CL_DEVICE_OPENCL_C_VERSION
        ),
        (get_extensions, String, cl3::device::CL_DEVICE_EXTENSIONS),
    );

    #[cfg(feature = "CL_VERSION_1_2")]
    cl_device_generate_getters!(
        (
            get_double_fp_config,
            u32,
            cl3::device::CL_DEVICE_DOUBLE_FP_CONFIG
        ),
        (
            get_linker_available,
            bool,
            cl3::device::CL_DEVICE_LINKER_AVAILABLE
        ),
        (
            get_built_in_kernels,
            String,
            cl3::device::CL_DEVICE_BUILT_IN_KERNELS
        ),
        (
            get_image_max_buffer_size,
            usize,
            cl3::device::CL_DEVICE_IMAGE_MAX_BUFFER_SIZE
        ),
        (
            get_image_max_array_size,
            usize,
            cl3::device::CL_DEVICE_IMAGE_MAX_ARRAY_SIZE
        ),
        (
            get_printf_buffer_size,
            usize,
            cl3::device::CL_DEVICE_PRINTF_BUFFER_SIZE
        ),
        (
            get_preferred_interop_user_sync,
            bool,
            cl3::device::CL_DEVICE_PREFERRED_INTEROP_USER_SYNC
        ),
        (
            get_partition_max_sub_devices,
            u32,
            cl3::device::CL_DEVICE_PARTITION_MAX_SUB_DEVICES
        ),
        (
            get_partition_affinity_domain,
            u32,
            cl3::device::CL_DEVICE_PARTITION_AFFINITY_DOMAIN
        ),
        (
            get_reference_count,
            u32,
            cl3::device::CL_DEVICE_REFERENCE_COUNT
        ),
    );

    #[cfg(feature = "CL_VERSION_2_0")]
    cl_device_generate_getters!(
        (
            get_max_read_write_image_args,
            u32,
            cl3::device::CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS
        ),
        (
            get_image_pitch_alignment,
            u32,
            cl3::device::CL_DEVICE_IMAGE_PITCH_ALIGNMENT
        ),
        (
            get_image_base_address_alignment,
            u32,
            cl3::device::CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT
        ),
        (get_max_pipe_args, u32, cl3::device::CL_DEVICE_MAX_PIPE_ARGS),
        (
            get_pipe_max_active_reservations,
            u32,
            cl3::device::CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS
        ),
        (
            get_pipe_max_packet_size,
            u32,
            cl3::device::CL_DEVICE_PIPE_MAX_PACKET_SIZE
        ),
        (
            get_max_global_variable_size,
            usize,
            cl3::device::CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE
        ),
        (
            get_global_variable_preferred_total_size,
            usize,
            cl3::device::CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE
        ),
        (
            get_queue_on_host_properties,
            u32,
            cl3::device::CL_DEVICE_QUEUE_ON_HOST_PROPERTIES
        ),
        (
            get_queue_on_device_properties,
            u32,
            cl3::device::CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES
        ),
        (
            get_queue_on_device_preferred_size,
            u32,
            cl3::device::CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE
        ),
        (
            get_queue_on_device_max_size,
            u32,
            cl3::device::CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE
        ),
        (
            get_max_on_device_queues,
            u32,
            cl3::device::CL_DEVICE_MAX_ON_DEVICE_QUEUES
        ),
        (
            get_max_on_device_events,
            u32,
            cl3::device::CL_DEVICE_MAX_ON_DEVICE_EVENTS
        ),
        (
            get_svm_capabilities,
            u32,
            cl3::device::CL_DEVICE_SVM_CAPABILITIES
        ),
        (
            get_preferred_platform_atomic_alignment,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT
        ),
        (
            get_preferred_global_atomic_alignment,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT
        ),
        (
            get_preferred_local_atomic_alignment,
            u32,
            cl3::device::CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT
        ),
    );

    #[cfg(feature = "CL_VERSION_2_1")]
    cl_device_generate_getters!(
        (get_il_version, String, cl3::device::CL_DEVICE_IL_VERSION),
        (
            get_max_num_sub_groups,
            u32,
            cl3::device::CL_DEVICE_MAX_NUM_SUB_GROUPS
        ),
        (
            get_sub_group_independent_forward_progress,
            bool,
            cl3::device::CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS
        ),
    );

    #[cfg(feature = "CL_VERSION_3_0")]
    cl_device_generate_getters!(
        (
            get_numeric_version,
            u32,
            cl3::device::CL_DEVICE_NUMERIC_VERSION
        ),
        (
            get_atomic_memory_capabilities,
            u32,
            cl3::device::CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES
        ),
        (
            get_atomic_fence_capabilities,
            u32,
            cl3::device::CL_DEVICE_ATOMIC_FENCE_CAPABILITIES
        ),
        (
            get_non_uniform_work_group_support,
            bool,
            cl3::device::CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT
        ),
        (
            get_work_group_collective_functions_support,
            bool,
            cl3::device::CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT
        ),
        (
            get_generic_address_space_support,
            bool,
            cl3::device::CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT
        ),
        (
            get_device_enqueue_capabilities,
            u32,
            cl3::device::CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES
        ),
        (get_pipe_support, bool, cl3::device::CL_DEVICE_PIPE_SUPPORT),
        (
            get_preferred_work_group_size_multiple,
            usize,
            cl3::device::CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE
        ),
        (
            get_latest_conformance_version_passed,
            String,
            cl3::device::CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED
        ),
    );
}

impl Releaseable for ClDevice {
    unsafe fn increase_reference_count(&self) {
        unsafe {
            cl3::device::retain_device(self.value);
        }
    }
}

impl Drop for ClDevice {
    fn drop(&mut self) {
        unsafe {
            cl3::device::release_device(self.value);
        }
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
impl std::fmt::Display for ClDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClDevice {{ name: {}, vendor: {}, version: {} }}",
            self.get_name().unwrap_or_default(),
            self.get_vendor().unwrap_or_default(),
            self.get_version().unwrap_or_default()
        )
    }
}

impl Clone for ClDevice {
    fn clone(&self) -> Self {
        unsafe {
            self.increase_reference_count();
        }

        Self {
            value: self.value.clone(),
        }
    }
}
