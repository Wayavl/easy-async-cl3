use easy_async_opencl3::{
    async_executor::AsyncExecutor,
    cl_types::{
        cl_device::opencl_version::OpenCLVersion, cl_pipe::ClPipe, memory_flags::MemoryFlags,
    },
    error::ClError,
};
use std::os::raw::c_void;

#[tokio::test]
async fn test_pipes_producer_consumer() -> Result<(), ClError> {
    let executor = AsyncExecutor::new_best_platform()?;
    let device = executor
        .get_context()
        .get_devices()?
        .first()
        .cloned()
        .unwrap();

    // 1. Verificación de soporte
    if device.get_opencl_version() < OpenCLVersion::V2_0
        || !device.get_pipe_support().unwrap_or(false)
    {
        println!(
            "Saltando test de Pipes: Hardware no compatible (se requiere OpenCL 2.0+ y soporte de Pipes)."
        );
        return Ok(());
    }

    println!(
        "Iniciando test de Pipes en {}",
        device.get_name().unwrap_or_default()
    );

    // 2. Programas (Producer y Consumer)
    let kernel_code = r#"
        kernel void producer(write_only pipe int out_pipe, int value) {
            // write_pipe devuelve 0 si tiene éxito
            int res = write_pipe(out_pipe, &value);
        }

        kernel void consumer(read_only pipe int in_pipe, global int* out_buf) {
            int value;
            // read_pipe devuelve 0 si tiene éxito
            int res = read_pipe(in_pipe, &value);
            if (res == 0) {
                *out_buf = value;
            } else {
                *out_buf = -1;
            }
        }
    "#;

    let program = executor.build_program(kernel_code.to_string(), Some("-cl-std=CL2.0"))?;
    let producer_kernel = executor.create_kernel(&program, "producer")?;
    let consumer_kernel = executor.create_kernel(&program, "consumer")?;

    // 3. Recursos
    // Pipe de enteros, máximo 1024 elementos
    let pipe = ClPipe::new(
        executor.get_context().as_ref(),
        &[MemoryFlags::ReadWrite],
        4,
        1024,
    )?;

    let mut result_data = vec![0i32; 1];
    let result_buffer = executor.create_buffer(
        &[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr],
        4,
        result_data.as_mut_ptr() as *mut c_void,
    )?;

    // 4. Ejecución
    let test_value = 12345i32;

    println!("Ejecutando Producer...");
    executor
        .create_task(&producer_kernel)
        .arg_pipe(0, &pipe)
        .arg_scalar(1, test_value)
        .global_work_dims(1, 1, 1)
        .run()
        .await?;

    println!("Ejecutando Consumer...");
    executor
        .create_task(&consumer_kernel)
        .arg_pipe(0, &pipe)
        .arg_buffer(1, &result_buffer)
        .global_work_dims(1, 1, 1)
        .read_buffer(&result_buffer, &mut result_data)
        .run()
        .await?;

    // 5. Verificación
    println!("Resultado obtenido del pipe: {}", result_data[0]);
    assert_eq!(result_data[0], test_value);

    Ok(())
}
