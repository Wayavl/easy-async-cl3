use easy_async_opencl3::{
    async_executor::AsyncExecutor,
    cl_types::memory_flags::MemoryFlags,
    error::ClError,
};

#[tokio::test]
async fn test_buffer_read_write() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    let size = 1024;
    let mut data_source: Vec<f32> = (0..size).map(|i| i as f32).collect();
    let mut data_dest = vec![0.0f32; size];

    // Create a buffer
    let buffer = executor.create_buffer(
        &[MemoryFlags::ReadWrite], 
        size * 4, 
        std::ptr::null_mut()
    )?;

    // Write data to buffer using AsyncExecutor::write_buffer
    println!("Writing data to buffer...");
    let _write_event = executor.write_buffer(&buffer, &mut data_source).await?;
    
    // We can explicitly wait for clarity, though await on write_buffer usually awaits the future of the event logic
    // write_event.wait()?;

    // Read data from buffer using AsyncExecutor::read_buffer
    println!("Reading data from buffer...");
    let _read_event = executor.read_buffer(&buffer, &mut data_dest).await?;
    // read_event.wait()?;

    // Verify
    for i in 0..size {
        if (data_source[i] - data_dest[i]).abs() > f32::EPSILON {
            panic!("Mismatch at index {}: source {} != dest {}", i, data_source[i], data_dest[i]);
        }
    }
    println!("Buffer read/write verification successful!");
    Ok(())
}
