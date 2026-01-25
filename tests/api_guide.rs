use easy_async_opencl3::{
    async_executor::AsyncExecutor,
    cl_types::memory_flags::MemoryFlags,
    error::ClError,
};
use std::os::raw::c_void;

/// # LEARNING GUIDE: Vector Addition
#[tokio::test]
async fn guide_01_vector_addition() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    let kernel_code = r#"
        kernel void add(global float* a, const global float* b) {
            size_t id = get_global_id(0);
            a[id] += b[id];
        }
    "#;
    let program = executor.build_program(kernel_code.to_string(), None)?;
    let kernel = executor.create_kernel(&program, "add")?;

    let size = 1024;
    let mut data_a = vec![1.0f32; size];
    let data_b = vec![2.0f32; size];

    let buf_a = executor.create_buffer(
        &[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], 
        size * 4, 
        data_a.as_mut_ptr() as *mut c_void
    )?;
    let buf_b = executor.create_buffer(
        &[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr], 
        size * 4, 
        data_b.as_ptr() as *mut c_void
    )?;

    executor.create_task(&kernel)
        .arg_buffer(0, &buf_a)
        .arg_buffer(1, &buf_b)
        .global_work_dims(size, 1, 1) // We use global, but don't specify local...
        .read_buffer(&buf_a, &mut data_a)
        .run() // ...Auto-tuning will decide the local_work_size for us!
        .await?;

    assert_eq!(data_a[0], 3.0);
    println!("Guide 01: Success! 1.0 + 2.0 = {}", data_a[0]);
    Ok(())
}

/// # LEARNING GUIDE: Shared Virtual Memory (SVM)
#[tokio::test]
async fn guide_02_svm_basics() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    let device = executor.get_context().get_devices()?.first().cloned().unwrap();
    if device.get_opencl_version() < easy_async_opencl3::cl_types::cl_device::opencl_version::OpenCLVersion::V2_0 {
        println!("{}", device.get_opencl_version());
        return Ok(());
    }

    let kernel_code = "kernel void fill(global float* data) { data[get_global_id(0)] = 42.0; }";
    let program = executor.build_program(kernel_code.to_string(), None)?;
    let kernel = executor.create_kernel(&program, "fill")?;

    let size = 512;
    let mut svm_buffer = executor.create_svm_buffer::<f32>(&[MemoryFlags::ReadWrite], size)?;

    executor.create_task(&kernel)
        .arg_svm(0, &svm_buffer)
        .global_work_dims(size, 1, 1)
        .run()
        .await?;

    let queue = &executor.get_queues()[0];
    let guard = svm_buffer.map_mut(queue, &vec![MemoryFlags::ReadOnly])?;
    println!("Guide 02: Success! SVM value is {}", guard[0]);
    assert_eq!(guard[0], 42.0);
    
    Ok(())
}

/// # LEARNING GUIDE: Professional Error Handling
/// Shows how errors are now descriptive and easy to diagnose.
#[tokio::test]
async fn guide_04_error_handling() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    
    // Force a compilation error
    let bad_code = "kernel void fail() { this_is_not_c_code }";
    let result = executor.build_program(bad_code.to_string(), None);
    
    if let Err(e) = result {
        // The error now implements Display and is very descriptive
        println!("Expected Error caught: {}", e);
        // We can verify if it's a specific API error
        match e {
            ClError::Api(api_err) => {
                println!("Confirmed API Error type: {:?}", api_err);
            },
            _ => panic!("Expected API error"),
        }
    } else {
        panic!("Should have failed to compile");
    }

    Ok(())
}

/// # LEARNING GUIDE: Pipes (OpenCL 2.0)
/// Pipes allow sending data between kernels asynchronously.
#[tokio::test]
async fn guide_05_pipes_communication() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    let device = executor.get_context().get_devices()?.first().cloned().unwrap();
    
    if device.get_opencl_version() < easy_async_opencl3::cl_types::cl_device::opencl_version::OpenCLVersion::V2_0 || !device.get_pipe_support().unwrap_or(false) {
        println!("Pipes not supported, skipping.");
        return Ok(());
    }

    // Note: ClPipe is only available if the CL_VERSION_2_0 feature is active.
    #[cfg(feature = "CL_VERSION_2_0")]
    {
        use easy_async_opencl3::cl_types::cl_pipe::ClPipe;
        let _pipe = ClPipe::new(executor.get_context().as_ref(), &[MemoryFlags::ReadWrite], 4, 1024)?;
        println!("Guide 05: Pipe created successfully.");
    }
    
    Ok(())
}
