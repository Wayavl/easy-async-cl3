#[derive(Debug)]
pub enum ApiError {
    Unknown(i32),
    ClAcceleratorTypeNotSupportedIntel,
    ClBuildProgramFailure,
    ClCancelledImg,
    ClCommandTerminatedItselfWithFailureArm,
    ClCompilerNotAvailable,
    ClCompileProgramFailure,
    ClContextTerminatedKhr,
    ClD3D11ResourceAlreadyAcquiredKhr,
    ClD3D11ResourceNotAcquiredKhr,
    ClDeviceNotAvailable,
    ClDeviceNotFound,
    ClMemObjectAllocationFailure,
    ClOutOfResources,
    ClOutOfHostMemory,
    ClProfilingInfoNotAvailable,
    ClMemCopyOverlap,
    ClImageFormatMismatch,
    ClImageFormatNotSupported,
    ClMapFailure,
    ClMisalignedSubBufferOffset,
    ClExecStatusErrorForEventsInWaitList,
    ClLinkerNotAvailable,
    ClLinkProgramFailure,
    ClDevicePartitionFailed,
    ClKernelArgInfoNotAvailable,
    ClInvalidValue,
    ClInvalidDeviceType,
    ClInvalidPlatform,
    ClInvalidDevice,
    ClInvalidContext,
    ClInvalidQueueProperties,
    ClInvalidCommandQueue,
    ClInvalidHostPtr,
    ClInvalidMemObject,
    ClInvalidImageFormatDescriptor,
    ClInvalidImageSize,
    ClInvalidSampler,
    ClInvalidBinary,
    ClInvalidBuildOptions,
    ClInvalidProgram,
    ClInvalidProgramExecutable,
    ClInvalidKernelName,
    ClInvalidKernelDefinition,
    ClInvalidKernel,
    ClInvalidArgIndex,
    ClInvalidArgValue,
    ClInvalidArgSize,
    ClInvalidKernelArgs,
    ClInvalidWorkDimension,
    ClInvalidWorkGroupSize,
    ClInvalidWorkItemSize,
    ClInvalidGlobalOffset,
    ClInvalidEventWaitList,
    ClInvalidEvent,
    ClInvalidOperation,
    ClInvalidGlObject,
    ClInvalidBufferSize,
    ClInvalidMipLevel,
    ClInvalidGlobalWorkSize,
    ClInvalidProperty,
    ClInvalidImageDescriptor,
    ClInvalidCompilerOptions,
    ClInvalidLinkerOptions,
    ClInvalidDevicePartitionCount,
    ClInvalidPipeSize,
    ClInvalidDeviceQueue,
    ClInvalidSpecId,
    ClMaxSizeRestrictionExceeded,
}

impl ApiError {
    pub fn get_error(code: i32) -> ApiError {
        match code {
            cl3::error_codes::CL_ACCELERATOR_TYPE_NOT_SUPPORTED_INTEL => {
                ApiError::ClAcceleratorTypeNotSupportedIntel
            }
            cl3::error_codes::CL_BUILD_PROGRAM_FAILURE => ApiError::ClBuildProgramFailure,
            cl3::error_codes::CL_CANCELLED_IMG => ApiError::ClCancelledImg,
            cl3::error_codes::CL_COMMAND_TERMINATED_ITSELF_WITH_FAILURE_ARM => {
                ApiError::ClCommandTerminatedItselfWithFailureArm
            }
            cl3::error_codes::CL_COMPILER_NOT_AVAILABLE => ApiError::ClCompilerNotAvailable,
            cl3::error_codes::CL_COMPILE_PROGRAM_FAILURE => ApiError::ClCompileProgramFailure,
            cl3::error_codes::CL_CONTEXT_TERMINATED_KHR => ApiError::ClContextTerminatedKhr,
            cl3::error_codes::CL_D3D11_RESOURCE_ALREADY_ACQUIRED_KHR => {
                ApiError::ClD3D11ResourceAlreadyAcquiredKhr
            }
            cl3::error_codes::CL_D3D11_RESOURCE_NOT_ACQUIRED_KHR => {
                ApiError::ClD3D11ResourceNotAcquiredKhr
            }
            cl3::error_codes::CL_DEVICE_NOT_AVAILABLE => ApiError::ClDeviceNotAvailable,
            cl3::error_codes::CL_DEVICE_NOT_FOUND => ApiError::ClDeviceNotFound,
            cl3::error_codes::CL_MEM_OBJECT_ALLOCATION_FAILURE => {
                ApiError::ClMemObjectAllocationFailure
            }
            cl3::error_codes::CL_OUT_OF_RESOURCES => ApiError::ClOutOfResources,
            cl3::error_codes::CL_OUT_OF_HOST_MEMORY => ApiError::ClOutOfHostMemory,
            cl3::error_codes::CL_PROFILING_INFO_NOT_AVAILABLE => {
                ApiError::ClProfilingInfoNotAvailable
            }
            cl3::error_codes::CL_MEM_COPY_OVERLAP => ApiError::ClMemCopyOverlap,
            cl3::error_codes::CL_IMAGE_FORMAT_MISMATCH => ApiError::ClImageFormatMismatch,
            cl3::error_codes::CL_IMAGE_FORMAT_NOT_SUPPORTED => ApiError::ClImageFormatNotSupported,
            cl3::error_codes::CL_MAP_FAILURE => ApiError::ClMapFailure,
            cl3::error_codes::CL_MISALIGNED_SUB_BUFFER_OFFSET => {
                ApiError::ClMisalignedSubBufferOffset
            }
            cl3::error_codes::CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST => {
                ApiError::ClExecStatusErrorForEventsInWaitList
            }
            cl3::error_codes::CL_LINKER_NOT_AVAILABLE => ApiError::ClLinkerNotAvailable,
            cl3::error_codes::CL_LINK_PROGRAM_FAILURE => ApiError::ClLinkProgramFailure,
            cl3::error_codes::CL_DEVICE_PARTITION_FAILED => ApiError::ClDevicePartitionFailed,
            cl3::error_codes::CL_KERNEL_ARG_INFO_NOT_AVAILABLE => {
                ApiError::ClKernelArgInfoNotAvailable
            }
            cl3::error_codes::CL_INVALID_VALUE => ApiError::ClInvalidValue,
            cl3::error_codes::CL_INVALID_DEVICE_TYPE => ApiError::ClInvalidDeviceType,
            cl3::error_codes::CL_INVALID_PLATFORM => ApiError::ClInvalidPlatform,
            cl3::error_codes::CL_INVALID_DEVICE => ApiError::ClInvalidDevice,
            cl3::error_codes::CL_INVALID_CONTEXT => ApiError::ClInvalidContext,
            cl3::error_codes::CL_INVALID_QUEUE_PROPERTIES => ApiError::ClInvalidQueueProperties,
            cl3::error_codes::CL_INVALID_COMMAND_QUEUE => ApiError::ClInvalidCommandQueue,
            cl3::error_codes::CL_INVALID_HOST_PTR => ApiError::ClInvalidHostPtr,
            cl3::error_codes::CL_INVALID_MEM_OBJECT => ApiError::ClInvalidMemObject,
            cl3::error_codes::CL_INVALID_IMAGE_FORMAT_DESCRIPTOR => {
                ApiError::ClInvalidImageFormatDescriptor
            }
            cl3::error_codes::CL_INVALID_IMAGE_SIZE => ApiError::ClInvalidImageSize,
            cl3::error_codes::CL_INVALID_SAMPLER => ApiError::ClInvalidSampler,
            cl3::error_codes::CL_INVALID_BINARY => ApiError::ClInvalidBinary,
            cl3::error_codes::CL_INVALID_BUILD_OPTIONS => ApiError::ClInvalidBuildOptions,
            cl3::error_codes::CL_INVALID_PROGRAM => ApiError::ClInvalidProgram,
            cl3::error_codes::CL_INVALID_PROGRAM_EXECUTABLE => ApiError::ClInvalidProgramExecutable,
            cl3::error_codes::CL_INVALID_KERNEL_NAME => ApiError::ClInvalidKernelName,
            cl3::error_codes::CL_INVALID_KERNEL_DEFINITION => ApiError::ClInvalidKernelDefinition,
            cl3::error_codes::CL_INVALID_KERNEL => ApiError::ClInvalidKernel,
            cl3::error_codes::CL_INVALID_ARG_INDEX => ApiError::ClInvalidArgIndex,
            cl3::error_codes::CL_INVALID_ARG_VALUE => ApiError::ClInvalidArgValue,
            cl3::error_codes::CL_INVALID_ARG_SIZE => ApiError::ClInvalidArgSize,
            cl3::error_codes::CL_INVALID_KERNEL_ARGS => ApiError::ClInvalidKernelArgs,
            cl3::error_codes::CL_INVALID_WORK_DIMENSION => ApiError::ClInvalidWorkDimension,
            cl3::error_codes::CL_INVALID_WORK_GROUP_SIZE => ApiError::ClInvalidWorkGroupSize,
            cl3::error_codes::CL_INVALID_WORK_ITEM_SIZE => ApiError::ClInvalidWorkItemSize,
            cl3::error_codes::CL_INVALID_GLOBAL_OFFSET => ApiError::ClInvalidGlobalOffset,
            cl3::error_codes::CL_INVALID_EVENT_WAIT_LIST => ApiError::ClInvalidEventWaitList,
            cl3::error_codes::CL_INVALID_EVENT => ApiError::ClInvalidEvent,
            cl3::error_codes::CL_INVALID_OPERATION => ApiError::ClInvalidOperation,
            cl3::error_codes::CL_INVALID_GL_OBJECT => ApiError::ClInvalidGlObject,
            cl3::error_codes::CL_INVALID_BUFFER_SIZE => ApiError::ClInvalidBufferSize,
            cl3::error_codes::CL_INVALID_MIP_LEVEL => ApiError::ClInvalidMipLevel,
            cl3::error_codes::CL_INVALID_GLOBAL_WORK_SIZE => ApiError::ClInvalidGlobalWorkSize,
            cl3::error_codes::CL_INVALID_PROPERTY => ApiError::ClInvalidProperty,
            cl3::error_codes::CL_INVALID_IMAGE_DESCRIPTOR => ApiError::ClInvalidImageDescriptor,
            cl3::error_codes::CL_INVALID_COMPILER_OPTIONS => ApiError::ClInvalidCompilerOptions,
            cl3::error_codes::CL_INVALID_LINKER_OPTIONS => ApiError::ClInvalidLinkerOptions,
            cl3::error_codes::CL_INVALID_DEVICE_PARTITION_COUNT => {
                ApiError::ClInvalidDevicePartitionCount
            }
            cl3::error_codes::CL_INVALID_PIPE_SIZE => ApiError::ClInvalidPipeSize,
            cl3::error_codes::CL_INVALID_DEVICE_QUEUE => ApiError::ClInvalidDeviceQueue,
            cl3::error_codes::CL_INVALID_SPEC_ID => ApiError::ClInvalidSpecId,
            cl3::error_codes::CL_MAX_SIZE_RESTRICTION_EXCEEDED => {
                ApiError::ClMaxSizeRestrictionExceeded
            }
            _ => ApiError::Unknown(code),
        }
    }
}
