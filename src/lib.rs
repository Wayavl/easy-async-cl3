//! # easy-async-cl3-ai
//! 
//! A high-level, async-first Rust wrapper for OpenCL with intelligent GPU management.
//! 
//! This library provides:
//! - **Async/await support**: All GPU operations return futures
//! - **Automatic resource management**: RAII-based cleanup
//! - **Multi-GPU support**: Automatic work distribution
//! - **Type-safe API**: Compile-time guarantees
//! - **Profiling support**: Built-in performance measurement
//! - **Modern OpenCL features**: Support for OpenCL 1.1 through 3.0

pub mod async_executor;
#[allow(unused)]
#[allow(dead_code)]
pub mod cl_types;
pub mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Instant;
    use std::{ffi::c_void, ptr::null_mut};

    use crate::{
        cl_types::{
            cl_command_queue::{
                ClCommandQueue,
                command_queue_parameters::{CommandQueueProperties, Version20},
            }, 
            cl_context::ClContext, 
            cl_platform::ClPlatform, 
            cl_image::{image_desc::ClImageDesc, image_formats::ClImageFormats},
            memory_flags::MemoryFlags
        },
        error::ClError,
    };

    macro_rules! time_it {
        ($label:expr, $block:block) => {{
            let start = Instant::now();
            let result = $block;
            let duration = start.elapsed();
            println!("[TIMER] {}: {:?}", $label, duration);
            result
        }};
    }

    #[test]
    fn test_hardware_discovery() -> Result<(), ClError> {
        println!("\n=== HARDWARE DISCOVERY ===");
        let platforms = time_it!("Platform enumeration", { ClPlatform::get_all()? });
        
        for (i, platform) in platforms.iter().enumerate() {
            println!("Platform [{}]: {}", i, platform);
            let devices = time_it!(format!("Device discovery (Platform {})", i), { platform.get_all_devices()? });
            for device in devices {
                println!("  - Device: {}", device);
                println!("    Version: {}", device.get_opencl_version());
                println!("    Max Compute Units: {}", device.get_max_compute_units().unwrap_or(0));
            }
        }
        Ok(())
    }

    #[test]
    fn test_core_resource_lifecycle() -> Result<(), ClError> {
        println!("\n=== RESOURCE LIFECYCLE ===");
        let platform = ClPlatform::default()?;
        let devices = platform.get_all_devices()?;
        
        let context = time_it!("Context creation", { ClContext::new(&devices)? });
        
        for (i, device) in devices.iter().enumerate() {
            let props = CommandQueueProperties::<Version20>::new()
                .set_cl_queue_properties(true, true, false, false)
                .get_properties();
            
            let _queue = time_it!(format!("Queue creation (Device {})", i), {
                ClCommandQueue::create_command_queue_with_properties(&context, device, &props)?
            });
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_operations_comprehensive() -> Result<(), ClError> {
        println!("\n=== MEMORY OPERATIONS ===");
        let platform = ClPlatform::default()?;
        let devices = platform.get_all_devices()?;
        let device = devices.first().unwrap();
        let executor = crate::async_executor::AsyncExecutor::new_from_devices(&devices)?;
        
        // 1. Buffer Test
        let size = 1024 * 1024; // 1MB
        let mut host_data: Vec<f32> = vec![42.0; size];
        time_it!("Buffer allocation (1MB) [High-level]", {
            executor.create_buffer(&[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], size * 4, host_data.as_mut_ptr() as *mut c_void)?
        });

        // 2. Image Test (Conditional)
        if device.get_image_support().unwrap_or(false) {
            let formats = ClImageFormats::rgba_unorm_int8();
            let desc = ClImageDesc {
                image_type: crate::cl_types::cl_image::image_type::ClImageType::Image2D,
                image_width: Some(512),
                image_height: Some(512),
                ..Default::default()
            };
            let _image = time_it!("Image creation (512x512 RGBA) [High-level]", {
                executor.create_image(&[MemoryFlags::ReadWrite], &formats, &desc, null_mut())?
            });
        }

        // 3. SVM Test (Conditional OpenCL 2.0+)
        if device.get_opencl_version() >= crate::cl_types::cl_device::opencl_version::OpenCLVersion::V2_0 {
            let svm = time_it!("SVM allocation (1024 f32) [High-level]", {
                executor.create_svm_buffer::<f32>(&[MemoryFlags::ReadWrite], 1024)?
            });
            if let Ok(svm_caps) = device.get_svm_capabilities() {
                println!("  SVM Capabilites: {:?}", svm_caps);
            }
            drop(svm);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_executor_full_pipeline() -> Result<(), ClError> {
        println!("\n=== EXECUTOR PIPELINE ===");
        let executor = time_it!("Executor initialization (Best Platform)", {
            crate::async_executor::AsyncExecutor::new_best_platform_with_options(true)?
        });
        
        let path = "./tests/program1test/add.cl";
        if !std::path::Path::new(path).exists() {
            println!("Skipping pipeline test: kernel file not found at {}", path);
            return Ok(());
        }

        let source = std::fs::read_to_string(path).unwrap();
        
        let builded = time_it!("Program build (High-level)", {
            executor.build_program(source, None)?
        });
        
        let kernel = time_it!("Kernel creation (High-level)", {
            executor.create_kernel(&builded, "add")?
        });

        let size = 1024 * 1024;
        let mut a: Vec<f32> = vec![1.0; size];
        let mut b: Vec<f32> = vec![2.0; size];
        let buffer_a = executor.create_buffer(&[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], size * 4, a.as_mut_ptr() as *mut c_void)?;
        let buffer_b = executor.create_buffer(&[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr], size * 4, b.as_mut_ptr() as *mut c_void)?;

        let report = time_it!("Task execution (AsyncExecutor)", {
            executor.create_task(kernel)
                .arg_buffer(0, &buffer_a)
                .arg_buffer(1, &buffer_b)
                .global_work_dims(size, 1, 1)
                .read_buffer(&buffer_a, &mut a)
                .run()
                .await?
        });

        println!("--- Profiling Results ---");
        println!("  Kernel Time (GPU): {} ns", report.total_kernel_duration_ns());
        println!("  Read Time (GPU):   {} ns", report.total_read_duration_ns());
        
        assert!((a[0] - 3.0).abs() < 1e-5);
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrency_stress() -> Result<(), ClError> {
        println!("\n=== CONCURRENCY STRESS ===");
        let executor = Arc::new(crate::async_executor::AsyncExecutor::new_best_platform()?);
        let path = "./tests/program1test/add.cl";
        if !std::path::Path::new(path).exists() { return Ok(()); }
        
        let source = std::fs::read_to_string(path).unwrap();
        let builded = executor.build_program(source, None)?;
        
        let mut tasks = Vec::new();
        for i in 0..10 {
            let kernel = executor.create_kernel(&builded, "add")?;
            let mut data = vec![i as f32; 1024];
            let buf = executor.create_buffer(&[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], 1024 * 4, data.as_mut_ptr() as *mut c_void)?;
            let buf_b = executor.create_buffer(&[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr], 1024 * 4, data.as_mut_ptr() as *mut c_void)?;
            
            let executor_clone = executor.clone();
            tasks.push(async move {
                executor_clone.create_task(kernel)
                    .arg_buffer(0, &buf)
                    .arg_buffer(1, &buf_b)
                    .global_work_dims(1024, 1, 1)
                    .run()
                    .await
            });
        }
        
        time_it!("10 Concurrent Tasks Submission", {
            futures::future::join_all(tasks).await;
        });
        
        Ok(())
    }

    #[tokio::test]
    async fn test_minimalist_api() -> Result<(), ClError> {
        println!("\n=== MINIMALIST API EXAMPLE ===");
        
        // 1. Initialize
        let executor = crate::async_executor::AsyncExecutor::new_best_platform()?;
        
        // 2. Build & Create Kernel
        let program = executor.build_program("kernel void add(global float* a, global float* b) { a[get_global_id(0)] += b[get_global_id(0)]; }".to_string(), None)?;
        let kernel = executor.create_kernel(&program, "add")?;
        
        // 3. Simple buffers
        let mut data: Vec<f32> = vec![10.0; 1024];
        let other: Vec<f32> = vec![5.0; 1024];
        let buf_a = executor.create_buffer(&[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], 1024 * 4, data.as_mut_ptr() as *mut c_void)?;
        let buf_b = executor.create_buffer(&[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr], 1024 * 4, other.as_ptr() as *mut c_void)?;
        
        // 4. Run declaratively
        executor.create_task(kernel)
            .arg_buffer(0, &buf_a)
            .arg_buffer(1, &buf_b)
            .global_work_dims(1024, 1, 1)
            .read_buffer(&buf_a, &mut data)
            .run()
            .await?;
            
        println!("Minimalist result: {}", data[0]);
        assert_eq!(data[0], 15.0);
        Ok(())
    }
}
