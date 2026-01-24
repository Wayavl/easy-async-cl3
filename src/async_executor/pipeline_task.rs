use crate::{
    async_executor::{kernel_arg::KernelArg, AsyncExecutor, task_builder::{TaskReport, OutputRead}},
    cl_types::{
        cl_event::ClEvent,
        cl_kernel::ClKernel,
        cl_buffer::ClBuffer,
        cl_image::ClImage,
        cl_svm_buffer::ClSvmBuffer,
    },
    error::ClError,
};
use std::os::raw::c_void;
use futures;

/// # PipelineStage
/// 
/// Represents a single kernel execution in a pipeline.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct PipelineStage<'a> {
    pub(crate) kernel: ClKernel,
    pub(crate) kernel_args: Vec<KernelArg<'a>>,
    pub(crate) global_work_dims: [usize; 3],
    pub(crate) global_work_offset: [usize; 3],
    pub(crate) local_work_dims: Option<[usize; 3]>,
}

/// # PipelineReport
/// 
/// Consolidated report for all stages in a pipeline.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct PipelineReport {
    pub stage_reports: Vec<TaskReport>,
}

impl PipelineReport {
    pub fn total_kernel_duration_ns(&self) -> u64 {
        self.stage_reports.iter().map(|r| r.total_kernel_duration_ns()).sum()
    }
}

/// # PipelineBuilder
/// 
/// Orchestrates the sequential execution of multiple kernels with automatic dependency management.
#[cfg(feature = "CL_VERSION_1_1")]
pub struct PipelineBuilder<'a> {
    async_executor: &'a AsyncExecutor,
    stages: Vec<PipelineStage<'a>>,
    final_reads: Vec<OutputRead<'a>>,
    profiling_enabled: bool,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(async_executor: &'a AsyncExecutor) -> Self {
        Self {
            async_executor,
            stages: Vec::new(),
            final_reads: Vec::new(),
            profiling_enabled: async_executor.is_profiling_enabled(),
        }
    }

    /// Starts defining a new stage in the pipeline.
    pub fn add_stage(self, kernel: ClKernel, x: usize, y: usize, z: usize) -> StageBuilder<'a> {
        StageBuilder {
            pipeline_builder: self,
            kernel,
            kernel_args: Vec::new(),
            global_work_dims: [x, y, z],
            global_work_offset: [0, 0, 0],
            local_work_dims: None,
        }
    }

    /// Adds a final read operation to the end of the pipeline.
    pub fn read_buffer<T>(mut self, buffer: &'a ClBuffer, host_memory: &mut [T]) -> Self {
        self.final_reads.push(OutputRead::Buffer {
            buffer,
            host_ptr: host_memory.as_mut_ptr() as *mut c_void,
            size: host_memory.len() * std::mem::size_of::<T>(),
        });
        self
    }

    pub async fn run(mut self) -> Result<PipelineReport, ClError> {
        let mut report = PipelineReport { stage_reports: Vec::new() };
        let mut last_events: Option<Vec<ClEvent>> = None;

        let stages = std::mem::take(&mut self.stages);

        for stage in stages {
            // We use the last stage's event as the wait list for this stage.
            let res = self.run_stage(stage, last_events).await?;
            last_events = Some(res.kernel_execution_events.clone());
            report.stage_reports.push(res);
        }

        // Final Reads
        if !self.final_reads.is_empty() {
            let queue = self.async_executor.get_optimal_queue();
            for read in self.final_reads {
                match read {
                    OutputRead::Buffer { buffer, host_ptr, size } => {
                        queue.enqueue_read_buffer_raw(buffer, None, host_ptr, size, last_events.clone()).await?;
                    }
                    #[cfg(feature = "CL_VERSION_1_2")]
                    OutputRead::Image { image, host_ptr, origin, region } => {
                        queue.read_image_raw(image, origin, region, 0, 0, host_ptr, last_events.clone()).await?;
                    }
                }
            }
        }

        Ok(report)
    }

    async fn run_stage(&self, stage: PipelineStage<'a>, wait_list: Option<Vec<ClEvent>>) -> Result<TaskReport, ClError> {
        // This is essentially a copy of TaskBuilder::run but with an external wait_list.
        // TODO: Refactor TaskBuilder to avoid this duplication.
        let mut report = TaskReport::new();
        let num_queues = self.async_executor.queues.len();
        
        let mut g_offset = stage.global_work_offset[0];
        let total_work = stage.global_work_dims[0];
        let total_weight: u64 = self.async_executor.weights.iter().sum();
        
        let mut futures = Vec::new();

        for i in 0..num_queues {
            let weight = self.async_executor.weights[i];
            let chunk_size = if i == num_queues - 1 {
                stage.global_work_offset[0] + total_work - g_offset
            } else {
                ((total_work as u128 * weight as u128) / total_weight as u128) as usize
            };

            if chunk_size == 0 && i != num_queues - 1 { continue; }

            let work_dim = if stage.global_work_dims[2] > 1 || stage.global_work_offset[2] > 0 { 3 }
                          else if stage.global_work_dims[1] > 1 || stage.global_work_offset[1] > 0 { 2 }
                          else { 1 };

            let g_offset_trimmed = vec![g_offset, stage.global_work_offset[1], stage.global_work_offset[2]][..work_dim].to_vec();
            let g_dims_trimmed = vec![chunk_size, stage.global_work_dims[1], stage.global_work_dims[2]][..work_dim].to_vec();
            
            let kernel = stage.kernel.clone();
            // In a pipeline, we MUST set args for EVERY enqueuing because we share the kernel object
            // across stages potentially, OR different queues.
            // Actually, OpenCL kernels hold their state. Set args must happen before enqueue.
            for arg in &stage.kernel_args {
                match arg {
                    KernelArg::Scalar { arg_index, arg } => {
                        unsafe { kernel.set_args(*arg_index, arg.len(), arg.as_ptr() as *const _)?; }
                    }
                    KernelArg::Buffer { arg_index, arg } => {
                        let handle = arg.as_ptr();
                        unsafe { kernel.set_args(*arg_index, 8, &handle as *const _ as *const _)?; }
                    }
                    #[cfg(feature = "CL_VERSION_1_2")]
                    KernelArg::Image { arg_index, arg } => {
                        let handle = arg.as_ptr();
                        unsafe { kernel.set_args(*arg_index, 8, &handle as *const _ as *const _)?; }
                    }
                    #[cfg(feature = "CL_VERSION_2_0")]
                    KernelArg::Svm { arg_index, arg, len: _ } => {
                        unsafe { kernel.set_svm_arg(*arg_index, 8, *arg)?; }
                    }
                    #[cfg(feature = "CL_VERSION_2_0")]
                    KernelArg::Pipe { arg_index, arg } => {
                        let handle = arg.as_ptr();
                        unsafe { kernel.set_args(*arg_index, 8, &handle as *const _ as *const _)?; }
                    }
                }
            }

            let queue = self.async_executor.queues[i].clone();
            let wl = wait_list.clone();
            
            // local_work_dims logic logic
            let l_dims_vec = if let Some(ld) = stage.local_work_dims {
                ld[..work_dim].to_vec()
            } else {
                 // Auto-tune logic (simplified version of TaskBuilder's)
                 Vec::new()
            };

            futures.push(async move {
                queue.enqueue_nd_range_kernel(
                    &kernel, 
                    work_dim as u32, 
                    g_offset_trimmed, 
                    g_dims_trimmed, 
                    l_dims_vec, 
                    None, 
                    wl
                ).await
            });

            g_offset += chunk_size;
        }

        let results = futures::future::join_all(futures).await;
        for res in results {
            let event = res?;
            if self.profiling_enabled {
                report.kernel_execution_events.push(event);
            }
        }

        Ok(report)
    }
}

/// # StageBuilder
/// 
/// Helper to configure a single stage within a pipeline.
pub struct StageBuilder<'a> {
    pipeline_builder: PipelineBuilder<'a>,
    kernel: ClKernel,
    kernel_args: Vec<KernelArg<'a>>,
    global_work_dims: [usize; 3],
    global_work_offset: [usize; 3],
    local_work_dims: Option<[usize; 3]>,
}

impl<'a> StageBuilder<'a> {
    pub fn arg_buffer(mut self, index: u32, buffer: &'a ClBuffer) -> Self {
        self.kernel_args.push(KernelArg::Buffer { arg_index: index, arg: buffer });
        self
    }

    pub fn arg_scalar<T>(mut self, index: u32, scalar: T) -> Self {
        let arg = unsafe {
            std::slice::from_raw_parts(&scalar as *const T as *const u8, std::mem::size_of::<T>()).to_vec()
        };
        self.kernel_args.push(KernelArg::Scalar { arg_index: index, arg });
        self
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn arg_image(mut self, index: u32, image: &'a ClImage) -> Self {
        self.kernel_args.push(KernelArg::Image { arg_index: index, arg: image });
        self
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn arg_svm<T>(mut self, index: u32, buffer: &'a ClSvmBuffer<T>) -> Self {
        self.kernel_args.push(KernelArg::Svm { arg_index: index, arg: buffer.as_ptr(), len: buffer.len });
        self
    }

    pub fn local_work_dims(mut self, x: usize, y: usize, z: usize) -> Self {
        self.local_work_dims = Some([x, y, z]);
        self
    }

    pub fn global_work_offset(mut self, x: usize, y: usize, z: usize) -> Self {
        self.global_work_offset = [x, y, z];
        self
    }

    /// Finalizes this stage and returns to the pipeline builder.
    pub fn finish(self) -> PipelineBuilder<'a> {
        let mut pb = self.pipeline_builder;
        let stage = PipelineStage {
            kernel: self.kernel,
            kernel_args: self.kernel_args,
            global_work_dims: self.global_work_dims,
            global_work_offset: self.global_work_offset,
            local_work_dims: self.local_work_dims,
        };
        pb.stages.push(stage);
        pb
    }
}
