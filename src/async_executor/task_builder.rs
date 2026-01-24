use crate::{async_executor::{AsyncExecutor, kernel_arg::KernelArg}, cl_types::{cl_buffer::ClBuffer, cl_device::opencl_version::OpenCLVersion, cl_event::ClEvent, cl_image::ClImage, cl_kernel::ClKernel, cl_svm_buffer::ClSvmBuffer, cl_pipe::ClPipe}, error::ClError};
use std::os::raw::c_void;
use futures;

#[cfg(feature = "CL_VERSION_1_1")]
pub enum OutputRead<'a> {
    Buffer {
        buffer: &'a ClBuffer,
        host_ptr: *mut c_void,
        size: usize,
    },
    #[cfg(feature = "CL_VERSION_1_2")]
    Image {
        image: &'a ClImage,
        host_ptr: *mut c_void,
        origin: [usize; 3],
        region: [usize; 3],
    }
}

#[cfg(feature = "CL_VERSION_1_1")]
unsafe impl<'a> Send for OutputRead<'a> {}
#[cfg(feature = "CL_VERSION_1_1")]
unsafe impl<'a> Sync for OutputRead<'a> {}

/// # TaskReport
/// 
/// Contains the results of a task execution.
/// If you enabled profiling, you'll find the exact execution times here.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct TaskReport {
    /// Kernel execution events (one per GPU used).
    pub kernel_execution_events: Vec<ClEvent>,
    /// Memory read events (when you use .read_buffer()).
    pub read_events: Vec<ClEvent>,
}

#[cfg(feature = "CL_VERSION_1_1")]
impl TaskReport {
    pub fn new() -> Self {
        Self {
            kernel_execution_events: Vec::new(),
            read_events: Vec::new(),
        }
    }

    /// Calculates the total time the kernels were running on the GPU.
    pub fn total_kernel_duration_ns(&self) -> u64 {
        self.kernel_execution_events.iter().filter_map(|e| e.get_duration_nanos().ok()).sum()
    }

    /// Calculates the total time it took to read the data back to the CPU.
    pub fn total_read_duration_ns(&self) -> u64 {
        self.read_events.iter().filter_map(|e| e.get_duration_nanos().ok()).sum()
    }
}

/// # TaskBuilder
/// 
/// A tool for configuring and launching tasks on the GPU.
/// 
/// You use this "builder" to specify what data the kernel needs (arguments),
/// how much work needs to be done (dimensions), and what data you want to read when finished.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct TaskBuilder<'a> {
    async_executor: &'a AsyncExecutor,
    kernel: ClKernel,
    kernel_args: Vec<KernelArg<'a>>,
    global_work_dims: Option<[usize; 3]>,
    global_work_offset: Option<[usize; 3]>,
    local_work_dims: Option<[usize; 3]>,
    output_reads: Vec<OutputRead<'a>>,
    wait_list: Option<Vec<ClEvent>>,
    profiling_enabled: bool,
}


impl<'a> TaskBuilder<'a> {
    pub fn new(async_executor: &'a AsyncExecutor, kernel: ClKernel) -> Self {
        Self {
            async_executor,
            kernel,
            kernel_args: Vec::new(),
            local_work_dims: None,
            global_work_dims: None,
            global_work_offset: None,
            output_reads: Vec::new(),
            wait_list: None,
            profiling_enabled: async_executor.is_profiling_enabled(),
        }
    }

    pub fn with_profiling(mut self, enabled: bool) -> Self {
        self.profiling_enabled = enabled;
        self
    }

    /// Passes a simple value (like an int or float) to the kernel.
    pub fn arg_scalar<T>(self, arg_index: u32, scalar: T) -> Self {
        self.add_scalar(arg_index, scalar, std::mem::size_of::<T>())
    }

    /// Passes a memory Buffer to the kernel.
    pub fn arg_buffer(self, arg_index: u32, buffer: &'a ClBuffer) -> Self {
        self.add_buffer(arg_index, buffer)
    }

    pub fn arg_image(self, arg_index: u32, image: &'a ClImage) -> Self {
        self.add_image_buffer(arg_index, image)
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn arg_svm<T>(self, arg_index: u32, buffer: &'a ClSvmBuffer<T>) -> Self {
        self.add_svm_buffer(arg_index, buffer)
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn arg_pipe(self, arg_index: u32, pipe: &'a ClPipe) -> Self {
        self.add_pipe(arg_index, pipe)
    }

    /// Indicates that when the kernel finishes, you want this buffer to be copied
    /// automatically to your program's memory (host_memory).
    pub fn read_buffer<T>(mut self, buffer: &'a ClBuffer, host_memory: &mut [T]) -> Self {
        self.output_reads.push(OutputRead::Buffer {
            buffer,
            host_ptr: host_memory.as_mut_ptr() as *mut c_void,
            size: host_memory.len() * std::mem::size_of::<T>(),
        });
        self
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn read_image<T>(mut self, image: &'a ClImage, host_memory: &mut [T], origin: [usize; 3], region: [usize; 3]) -> Self {
        self.output_reads.push(OutputRead::Image {
            image,
            host_ptr: host_memory.as_mut_ptr() as *mut c_void,
            origin,
            region,
        });
        self
    }

    pub fn add_scalar<T>(mut self, arg_index: u32,scalar: T, byte_size: usize) -> Self {
        let kernel_arg = KernelArg::Scalar {
            arg_index,
            arg: unsafe {
                std::slice::from_raw_parts(&scalar as *const T as *const u8, byte_size).to_vec()
            }
        };
        self.kernel_args.push(kernel_arg);
        self
    }

    pub fn add_buffer(mut self, arg_index: u32, buffer: &'a ClBuffer) -> Self {
        let kernel_arg = KernelArg::Buffer { arg_index, arg: buffer };
        self.kernel_args.push(kernel_arg);
        self
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn add_svm_buffer<T>(mut self, arg_index: u32, buffer: &'a ClSvmBuffer<T>) -> Self {
        let kernel_arg = KernelArg::Svm { arg_index, arg: buffer.as_ptr(), len: buffer.len };
        self.kernel_args.push(kernel_arg);
        self
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn add_image_buffer(mut self, arg_index: u32, image_buffer: &'a ClImage) -> Self {
        let kernel_arg = KernelArg::Image { arg_index, arg: image_buffer };
        self.kernel_args.push(kernel_arg);
        self
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn add_pipe(mut self, arg_index: u32, pipe: &'a ClPipe) -> Self {
        let kernel_arg = KernelArg::Pipe { arg_index, arg: pipe };
        self.kernel_args.push(kernel_arg);
        self
    }

    pub fn local_work_dims(mut self, x: usize, y: usize, z: usize) -> Self {
        self.local_work_dims = Some([x,y,z]);
        self
    }

    pub fn global_work_offset(mut self, x: usize, y: usize, z: usize) -> Self {
        self.global_work_offset = Some([x,y,z]);
        self
    }

    /// Defines how many "instances" of the kernel will be executed in total.
    /// If you don't specify the local size (local_work_dims), the library will optimize it for you.
    pub fn global_work_dims(mut self, x: usize, y: usize, z: usize) -> Self {
        self.global_work_dims = Some([x,y,z]);
        self
    }

    pub fn add_wait_list(mut self, wait_list: Vec<ClEvent>) -> Self {
        self.wait_list = Some(wait_list);
        self
    }

    /// Executes the task asynchronously.
    /// 
    /// This function:
    /// 1. Configures the arguments on the GPU.
    /// 2. Distributes the work if you have multiple cards.
    /// 3. Launches the kernels.
    /// 4. Waits for them to finish (without blocking your CPU thread).
    /// 5. Reads the results back.
    pub async fn run(self) -> Result<TaskReport, ClError> {
        let mut report = TaskReport::new();
        let num_queues = self.async_executor.queues.len();
        if num_queues == 0 {
            return Err(ClError::Wrapper(crate::error::wrapper_error::WrapperError::PlatformsNotFound));
        }

        // Set kernel arguments
        for arg in &self.kernel_args {
            match arg {
                KernelArg::Scalar { arg_index, arg } => {
                    unsafe {
                        self.kernel.set_args(*arg_index, arg.len(), arg.as_ptr() as *const _)?;
                    }
                }
                KernelArg::Buffer { arg_index, arg } => {
                    let handle = arg.as_ptr();
                    unsafe {
                        self.kernel.set_args(*arg_index, std::mem::size_of::<*mut std::os::raw::c_void>(), &handle as *const _ as *const _)?;
                    }
                }
                KernelArg::Svm { arg_index, arg, len } => {
                    unsafe {
                        self.kernel.set_svm_arg(*arg_index, *len, *arg)?;
                    }
                }
                KernelArg::Image { arg_index, arg } => {
                    let handle = arg.as_ptr();
                    unsafe {
                        self.kernel.set_args(*arg_index, std::mem::size_of::<*mut std::os::raw::c_void>(), &handle as *const _ as *const _)?;
                    }
                }
                #[cfg(feature = "CL_VERSION_2_0")]
                KernelArg::Pipe { arg_index, arg } => {
                    let handle = arg.as_ptr();
                    unsafe {
                        self.kernel.set_args(*arg_index, std::mem::size_of::<*mut std::os::raw::c_void>(), &handle as *const _ as *const _)?;
                    }
                }
            }
        }

        let global_work_dims = self.global_work_dims.unwrap_or([1, 1, 1]);
        let global_work_offset = self.global_work_offset.unwrap_or([0, 0, 0]);

        let total_work = global_work_dims[0];
        let total_weight: u64 = self.async_executor.weights.iter().sum();
        
        let mut futures = Vec::new();
        let mut current_offset = global_work_offset[0];

        for i in 0..num_queues {
            let weight = self.async_executor.weights[i];
            
            // Calculate chunk size based on weight
            // The last device takes whatever is left to avoid rounding errors
            let chunk_size = if i == num_queues - 1 {
                global_work_offset[0] + total_work - current_offset
            } else {
                ((total_work as u128 * weight as u128) / total_weight as u128) as usize
            };

            if chunk_size == 0 && i != num_queues - 1 {
                continue;
            }

            let g_offset = vec![current_offset, global_work_offset[1], global_work_offset[2]];
            let g_dims = vec![chunk_size, global_work_dims[1], global_work_dims[2]];
            
            // Infer work_dim from specified dims
            let work_dim = if global_work_dims[2] > 1 || global_work_offset[2] > 0 {
                3
            } else if global_work_dims[1] > 1 || global_work_offset[1] > 0 {
                2
            } else {
                1
            };

            let g_offset_trimmed = g_offset[..work_dim].to_vec();
            let g_dims_trimmed = g_dims[..work_dim].to_vec();
            let kernel_clone = self.kernel.clone();
            let queue = self.async_executor.queues[i].clone();
            let wait_list = self.wait_list.clone();

            let l_dims_trimmed = if let Some(ld) = self.local_work_dims {
                ld[..work_dim].to_vec()
            } else {
                // Auto-tune logic
                let version = self.async_executor.get_device_versions()[i];
                let device_res = self.async_executor.get_devices();
                
                if let Ok(devices) = device_res {
                    let device = devices[i].clone();
                    if version >= OpenCLVersion::V2_0 && device.get_non_uniform_work_group_support().unwrap_or(false) {
                        Vec::new() // NULL will let the driver decide with non-uniform support
                    } else if let Ok(preferred) = kernel_clone.get_work_group_size(device) {
                        // For simplicity, if 1D we use the preferred size
                        if work_dim == 1 {
                             vec![preferred]
                        } else {
                             Vec::new() // Safest choice for 2D/3D if we don't have factorization logic
                        }
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            };

            futures.push(async move {
                let event = queue.enqueue_nd_range_kernel(
                    &kernel_clone,
                    work_dim as u32,
                    g_offset_trimmed,
                    g_dims_trimmed,
                    l_dims_trimmed,
                    None,
                    wait_list
                ).await?;
                Ok::<ClEvent, ClError>(event)
            });

            current_offset += chunk_size;
        }

        let results = futures::future::join_all(futures).await;
        for res in results {
            let event = res?;
            if self.profiling_enabled {
                report.kernel_execution_events.push(event);
            }
        }

        // Automatic Reads
        if !self.output_reads.is_empty() {
            let queue = self.async_executor.queues.first().ok_or(ClError::Wrapper(crate::error::wrapper_error::WrapperError::PlatformsNotFound))?;
            for read in &self.output_reads {
                match read {
                    OutputRead::Buffer { buffer, host_ptr, size } => {
                        let event = queue.enqueue_read_buffer_raw(*buffer, None, *host_ptr, *size, None).await?;
                        if self.profiling_enabled {
                            report.read_events.push(event);
                        }
                    }
                    OutputRead::Image { image, host_ptr, origin, region } => {
                        let event = queue.read_image_raw(*image, *origin, *region, 0, 0, *host_ptr, None).await?;
                        if self.profiling_enabled {
                            report.read_events.push(event);
                        }
                    }
                }
            }
        }

        Ok(report)
    }
}

unsafe impl Sync for TaskBuilder<'_> {}
unsafe impl Send for TaskBuilder<'_> {}