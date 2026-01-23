mod task_builder;
mod kernel_arg;
use std::os::raw::c_void;
use std::sync::Arc;

use crate::{
    async_executor::task_builder::TaskBuilder, 
    cl_types::{
        cl_buffer::ClBuffer,
        cl_command_queue::{ClCommandQueue, command_queue_parameters::{CommandQueueProperties, Version20}},
        cl_context::ClContext, 
        cl_device::ClDevice, 
        cl_platform::ClPlatform,
        cl_kernel::ClKernel,
        cl_device::opencl_version::OpenCLVersion,
        cl_program::{ClProgram, Builded, NotBuilded, program_parameters::ProgramParameters},
        cl_image::{ClImage, image_desc::ClImageDesc, image_formats::ClImageFormats},
        cl_svm_buffer::ClSvmBuffer,
        memory_flags::MemoryFlags,
    }, 
    error::ClError
};

/// # AsyncExecutor
/// 
/// The `AsyncExecutor` is the central engine of this library. Its job is to simplify
/// all the "dirty work" of OpenCL: finding the best graphics card, creating the context,
/// managing command queues, and distributing work intelligently.
///
/// Think of it as an orchestra conductor that decides which musicians (GPUs) play each part.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct AsyncExecutor {
    context: Arc<ClContext>,
    queues: Vec<ClCommandQueue>,
    weights: Vec<u64>,
    profiling_enabled: bool,
    device_versions: Vec<OpenCLVersion>,
    devices: Vec<ClDevice>,
}

#[cfg(feature = "CL_VERSION_1_1")]
impl AsyncExecutor {
    //
    //
    // Static
    //
    //

    /// Creates an executor by automatically selecting the best available platform.
    /// 
    /// It will search among all cards (NVIDIA, AMD, Intel) and choose the one with
    /// the most computing power and memory.
    pub fn new_best_platform() -> Result<Self, ClError> {
        Self::new_best_platform_with_options(false)
    }

    /// Creates an executor with the best platform, allowing profiling to be enabled.
    /// 
    /// If `profiling_enabled` is true, you can measure exactly how many nanoseconds
    /// each kernel takes to execute on the card.
    pub fn new_best_platform_with_options(profiling_enabled: bool) -> Result<Self, ClError> {
        let platforms = ClPlatform::get_all()?;

        let scores: Vec<u64> = platforms
            .iter()
            .map(|f| Self::measure_platform_capacity(f))
            .collect::<Result<Vec<u64>, ClError>>()?;

        let best_score_index = {
            let mut idx: usize = 0;
            let mut max: u64 = 0;
            for (i, v) in scores.iter().enumerate() {
                if *v > max {
                    idx = i;
                    max = *v
                }
            }
            idx
        };

        let devices = match platforms.get(best_score_index) {
            Some(plat) => plat.get_all_devices()?,
            None => {
                return Err(ClError::Wrapper(
                    crate::error::wrapper_error::WrapperError::PlatformsNotFound,
                ));
            }
        };

        let context = Arc::new(ClContext::new(&devices)?);
        let mut queues = Vec::new();
        let mut weights = Vec::new();
        let mut device_versions = Vec::new();

        for device in &devices {
            let version = device.get_opencl_version();
            let mut supports_out_of_order = false;

            let queue = if version >= OpenCLVersion::V2_0 {
                if let Ok(host_props) = device.get_queue_on_host_properties() {
                    supports_out_of_order = (host_props as u64 & cl3::command_queue::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE) != 0;
                }
                
                let properties = CommandQueueProperties::<Version20>::new()
                    .set_cl_queue_properties(supports_out_of_order, profiling_enabled, false, false)
                    .get_properties();
                ClCommandQueue::create_command_queue_with_properties(&context, device, &properties)?
            } else {
                let mut properties = 0;
                if profiling_enabled {
                    properties |= cl3::command_queue::CL_QUEUE_PROFILING_ENABLE;
                }
                #[allow(deprecated)]
                ClCommandQueue::create_command_queue(&context, device, properties)?
            };
            
            queues.push(queue);
            weights.push(Self::measure_device_capacity(device)?);
            device_versions.push(version);
        }

        let executor = Self {
            context,
            queues,
            weights,
            device_versions,
            profiling_enabled,
            devices: devices.into_iter().map(|d| d.clone()).collect(),
        };

        Ok(executor)
    }

    pub fn new_all_platforms() -> Result<Self, ClError> {
        let platforms = ClPlatform::get_all()?;
        Self::new_from_platforms(&platforms)
    }

    pub fn new_from_platforms(platforms: &[ClPlatform]) -> Result<Self, ClError> {
        let mut all_devices = Vec::new();
        for platform in platforms {
            let devices = platform.get_all_devices()?;
            all_devices.extend(devices);
        }
        Self::new_from_devices(&all_devices)
    }

    pub fn new_from_devices(devices: &[ClDevice]) -> Result<Self, ClError> {
        Self::new_from_devices_with_options(devices, false)
    }

    pub fn new_from_devices_with_options(devices: &[ClDevice], profiling_enabled: bool) -> Result<Self, ClError> {
        let devices_vec = devices.to_vec();
        let context = Arc::new(ClContext::new(&devices_vec)?);
        let mut queues = Vec::new();
        let mut weights = Vec::new();
        let mut device_versions = Vec::new();

        for device in devices {
            let version = device.get_opencl_version();
            let mut supports_out_of_order = false;

            let queue = if version >= OpenCLVersion::V2_0 {
                if let Ok(host_props) = device.get_queue_on_host_properties() {
                    supports_out_of_order = (host_props as u64 & cl3::command_queue::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE) != 0;
                }

                let properties = CommandQueueProperties::<Version20>::new()
                    .set_cl_queue_properties(supports_out_of_order, profiling_enabled, false, false)
                    .get_properties();
                ClCommandQueue::create_command_queue_with_properties(&context, device, &properties)?
            } else {
                let mut properties = 0;
                if profiling_enabled {
                    properties |= cl3::command_queue::CL_QUEUE_PROFILING_ENABLE;
                }
                #[allow(deprecated)]
                ClCommandQueue::create_command_queue(&context, device, properties)?
            };

            queues.push(queue);
            weights.push(Self::measure_device_capacity(device)?);
            device_versions.push(version);
        }

        Ok(Self {
            context,
            queues,
            weights,
            device_versions,
            profiling_enabled,
            devices: devices_vec,
        })
    }

    pub fn is_profiling_enabled(&self) -> bool {
        self.profiling_enabled
    }

    pub fn get_context(&self) -> Arc<ClContext> {
        self.context.clone()
    }

    pub fn get_device_versions(&self) -> &[OpenCLVersion] {
        &self.device_versions
    }

    pub fn get_devices(&self) -> Result<Vec<ClDevice>, ClError> {
        Ok(self.devices.iter().map(|d| d.clone()).collect())
    }

    pub fn get_queues(&self) -> &[ClCommandQueue] {
        &self.queues
    }

    //
    // Engine (Self)
    //

    /// Creates a task to be executed.
    /// 
    /// This is the entry point for the declarative workflow.
    /// It receives a `ClKernel` (the function that will run on the GPU) and returns
    /// a `TaskBuilder` to configure the arguments and work size.
    pub fn create_task(&self, kernel: ClKernel) -> TaskBuilder<'_> {
        TaskBuilder::new(self, kernel)
    }

    //
    // Facade Methods (Simplifican la creación de recursos)
    //

    /// Compiles an OpenCL program from source code (C-like).
    /// 
    /// # Example
    /// ```rust
    /// let source = "kernel void add(...) { ... }";
    /// let program = executor.build_program(source.to_string(), None)?;
    /// ```
    pub fn build_program(&self, source: String, options: Option<&str>) -> Result<ClProgram<Builded>, ClError> {
        let unbuilded = ClProgram::<NotBuilded>::from_src(&self.context, source)?;
        let devices = self.context.get_devices()?;
        
        let params = match options {
            Some(opt) => ProgramParameters::default().custom(opt).get_parameters(),
            None => ProgramParameters::default().get_parameters(),
        };

    unbuilded.build(&params, &devices)
    }

    /// Compiles the program or loads it from binary if available.
    ///
    /// Checks if binaries exist in `binary_dest_folder`. If so, loads them.
    /// Otherwise, compiles from `src_path` and saves binaries to `binary_dest_folder`.
    pub fn compile_or_binary(
        &self,
        src_path: &str,
        binary_dest_folder: &str,
        options: Option<&str>,
    ) -> Result<ClProgram<Builded>, ClError> {
        use std::fs;
        use std::path::Path;
        use std::io::Read;
        use crate::error::wrapper_error::WrapperError;

        let path = Path::new(src_path);
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or(ClError::Wrapper(WrapperError::FailedToConvertStrToCString))?; 

        let devices = self.context.get_devices()?;
        
        let mut binaries: Vec<Vec<u8>> = Vec::new();
        let mut use_binaries = true;

        for (i, device) in devices.iter().enumerate() {
            let device_name = device.get_name()?.replace(" ", "_");
            let bin_filename = format!("{}_{}_{}.bin", file_stem, device_name, i);
            let bin_path = Path::new(binary_dest_folder).join(bin_filename);

            if bin_path.exists() {
                match fs::read(&bin_path) {
                    Ok(content) => binaries.push(content),
                    Err(_) => {
                        use_binaries = false;
                        break;
                    }
                }
            } else {
                use_binaries = false;
                break;
            }
        }

        if use_binaries && binaries.len() == devices.len() {
             let binary_slices: Vec<&[u8]> = binaries.iter().map(|b| b.as_slice()).collect();
             
             match ClProgram::<NotBuilded>::from_binary(&self.context, &devices, &binary_slices) {
                 Ok(program) => {
                    let params = match options {
                        Some(opt) => ProgramParameters::default().custom(opt).get_parameters(),
                        None => ProgramParameters::default().get_parameters(),
                    };
                    
                    match program.build(&params, &devices) {
                        Ok(built) => return Ok(built),
                        Err(_) => {
                            // Build from binary failed. Fallback to source.
                        }
                    }
                 },
                 Err(_) => {
                     // Failed to create from binary. Fallback.
                 }
             }
        }

        // Compile from source
        let mut src_content = String::new();
        fs::File::open(src_path)
            .map_err(|_| ClError::Wrapper(WrapperError::FileIOError))?
            .read_to_string(&mut src_content)
            .map_err(|_| ClError::Wrapper(WrapperError::FileIOError))?;

        let built_program = self.build_program(src_content, options)?;

        // Save binaries
        let _ = built_program.save_binary(binary_dest_folder, file_stem);

        Ok(built_program)
    }

    /// Creates a Kernel from an already compiled program.
    /// The `name` must exactly match the name of the `kernel` function in your C code.
    pub fn create_kernel(&self, program: &ClProgram<Builded>, name: &str) -> Result<ClKernel, ClError> {
        ClKernel::new(program, name)
    }

    /// Creates a memory Buffer on the GPU.
    /// 
    /// Buffers are "boxes" of data that the GPU can read or write.
    pub fn create_buffer(&self, flags: &[MemoryFlags], size: usize, host_ptr: *mut c_void) -> Result<ClBuffer, ClError> {
        ClBuffer::new(&self.context, &flags.to_vec(), size, host_ptr)
    }

    /// Creates an OpenCL Image (requires OpenCL 1.2+).
    /// Images are optimized for 2D/3D access and filtering.
    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn create_image(
        &self, 
        flags: &[MemoryFlags], 
        format: &ClImageFormats, 
        desc: &ClImageDesc, 
        host_ptr: *mut c_void
    ) -> Result<ClImage, ClError> {
        ClImage::new(&self.context, &flags.to_vec(), format, desc, host_ptr)
    }

    /// Creates an SVM Buffer (Shared Virtual Memory). (Requires OpenCL 2.0+).
    /// Allows sharing pointers directly between CPU and GPU without manual copies.
    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn create_svm_buffer<T>(&self, flags: &[MemoryFlags], len: usize) -> Result<ClSvmBuffer<T>, ClError> {
        ClSvmBuffer::<T>::new(&self.context, &flags.to_vec(), len, 0)
    }

    //
    //
    // Utils
    //
    //
    fn measure_platform_capacity(platform: &ClPlatform) -> Result<u64, ClError> {
        let mut score: u64 = 0;

        let devices = platform.get_all_devices()?;
        for device in &devices {
            score += Self::measure_device_capacity(device)?;
        }
        Ok(score)
    }

    fn measure_device_capacity(device: &ClDevice) -> Result<u64, ClError> {
        let compute_units = device.get_max_compute_units()?;
        let clock_frequency = device.get_max_clock_frequency()?;
        let memory = device.get_global_mem_size()? / (1024 * 1024);

        Ok(((compute_units as u64 * clock_frequency as u64) / 100) + (memory / 10))
    }
}

unsafe impl Sync for AsyncExecutor {}
unsafe impl Send for AsyncExecutor {}