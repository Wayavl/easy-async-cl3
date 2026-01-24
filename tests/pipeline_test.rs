use easy_async_opencl3::{
    async_executor::AsyncExecutor,
    cl_types::memory_flags::MemoryFlags,
    error::ClError,
};
use std::os::raw::c_void;

#[tokio::test]
async fn test_simple_pipeline() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    
    // 1. Kernels
    let add_src = "kernel void add(global float* a, global float* b) { size_t i = get_global_id(0); a[i] += b[i]; }";
    let mul_src = "kernel void mul(global float* a, float factor) { size_t i = get_global_id(0); a[i] *= factor; }";
    
    let p_add = executor.build_program(add_src.to_string(), None)?;
    let p_mul = executor.build_program(mul_src.to_string(), None)?;
    
    let k_add = executor.create_kernel(&p_add, "add")?;
    let k_mul = executor.create_kernel(&p_mul, "mul")?;
    
    // 2. Data
    let size = 1024;
    let mut data_a = vec![10.0f32; size];
    let data_b = vec![5.0f32; size];
    
    let buf_a = executor.create_buffer(&[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr], size * 4, data_a.as_mut_ptr() as *mut c_void)?;
    let buf_b = executor.create_buffer(&[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr], size * 4, data_b.as_ptr() as *mut c_void)?;
    
    // 3. Pipeline: (a + b) * 2.0
    // Result should be (10 + 5) * 2 = 30
    let report = executor.create_pipeline()
        .add_stage(k_add, size, 1, 1)
            .arg_buffer(0, &buf_a)
            .arg_buffer(1, &buf_b)
            .finish()
        .add_stage(k_mul, size, 1, 1)
            .arg_buffer(0, &buf_a)
            .arg_scalar(1, 2.0f32)
            .finish()
        .read_buffer(&buf_a, &mut data_a)
        .run()
        .await?;
        
    println!("Pipeline result: {}", data_a[0]);
    assert_eq!(data_a[0], 30.0);
    assert_eq!(report.stage_reports.len(), 2);
    
    Ok(())
}
