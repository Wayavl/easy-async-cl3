mod async_executor;
#[allow(unused)]
#[allow(dead_code)]
mod cl_types;
mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{ffi::c_void, fs::File, io::Write, thread::sleep};

    use cl3::platform;

    use crate::{
        cl_types::{
            buffer_flags::MemoryFlags,
            cl_buffer::ClBuffer,
            cl_command_queue::{
                ClCommandQueue,
                command_queue_parameters::{CommandQueueProperties, Version10, Version20},
            },
            cl_context::ClContext,
            cl_device::device_type::ALL,
            cl_kernel::ClKernel,
            cl_platform::ClPlatform,
            cl_program::{ClProgram, NotBuilded, program_parameters::ProgramParameters},
            cl_svm_buffer::ClSvmBuffer,
        },
        error::{ClError, api_error::ApiError, wrapper_error::WrapperError},
    };

    #[test]
    fn cl_platform_test() -> Result<(), ClError> {
        println!("All platforms: ");
        let get_all_paltforms = ClPlatform::get_all()?;
        get_all_paltforms.iter().for_each(|f| print!("{}\n", f));

        let default_platform = ClPlatform::default()?;
        println!("Default platform: {}", default_platform);
        println!(
            "Extensions: {}",
            default_platform.get_extensions().unwrap_or_default()
        );
        println!(
            "Extensions with version: {}",
            default_platform
                .get_extensions_with_version()
                .unwrap_or_default()
        );

        Ok(())
    }

    #[test]
    fn cl_device_test() -> Result<(), ClError> {
        let platforms = ClPlatform::get_all().unwrap();

        for platform in platforms {
            for device in platform.get_all_devices().unwrap() {
                println!("Device: {}", device);
                println!("  Name: {}", device.get_name().unwrap_or_default());
                println!("  Vendor: {}", device.get_vendor().unwrap_or_default());
                println!("  Version: {}", device.get_version().unwrap_or_default());
                println!(
                    "  Max Compute Units: {}",
                    device.get_max_compute_units().unwrap_or_default()
                );
                println!(
                    "  Global Memory Size: {}",
                    device.get_global_mem_size().unwrap_or_default()
                );
                println!(
                    "  Local Memory Size: {}",
                    device.get_local_mem_size().unwrap_or_default()
                );
                println!(
                    "  Max Work Group Size: {}",
                    device.get_max_work_group_size().unwrap_or_default()
                );
                println!(
                    "  Available: {}",
                    device.get_available().unwrap_or_default()
                );
                println!(
                    "  Max partition subdevice: {}",
                    device.get_partition_max_sub_devices().unwrap_or_default()
                );
                println!(
                    "  Clock frecuency: {}",
                    device.get_max_clock_frequency().unwrap_or_default()
                );
                println!(
                    "  Version number: {}",
                    device.get_numeric_version().unwrap_or_default()
                );
                println!("  Profile: {}", device.get_profile().unwrap_or_default());
            }
        }

        Ok(())
    }

    #[test]
    fn cl_sub_device_test() -> Result<(), ClError> {
        let default_platform = ClPlatform::default().unwrap();
        let all_devices = default_platform.get_all_devices().unwrap();

        for parent_device in all_devices {
            let subdevice = parent_device.create_subdevice_equally(4)?;
            for device in subdevice {
                println!("Device: {}", device);
                println!("  Name: {}", device.get_name().unwrap_or_default());
                println!("  Vendor: {}", device.get_vendor().unwrap_or_default());
                println!("  Version: {}", device.get_version().unwrap_or_default());
                println!(
                    "  Max Compute Units: {}",
                    device.get_max_compute_units().unwrap_or_default()
                );
                println!(
                    "  Global Memory Size: {}",
                    device.get_global_mem_size().unwrap_or_default()
                );
                println!(
                    "  Local Memory Size: {}",
                    device.get_local_mem_size().unwrap_or_default()
                );
                println!(
                    "  Max Work Group Size: {}",
                    device.get_max_work_group_size().unwrap_or_default()
                );
                println!(
                    "  Available: {}",
                    device.get_available().unwrap_or_default()
                );
            }
        }

        Ok(())
    }

    #[test]
    fn cl_context_test() -> Result<(), ClError> {
        let platform = ClPlatform::default().unwrap();
        let context_from_device_type = ClContext::new_from_device_type(&platform, ALL)?;
        let devices = platform.get_all_devices().unwrap();
        let context_from_default = ClContext::new(&devices)?;

        let context_array = vec![context_from_device_type, context_from_default];

        for platform in context_array {
            println!("Platform X");
            println!(
                "  Context reference count: {}",
                platform.get_context_reference_count().unwrap_or_default()
            );
            println!(
                "  Num devices: {}",
                platform.get_num_devices().unwrap_or_default()
            );
            for i in platform.get_devices()? {
                println!("  Device: X");
                println!("  Info: {}", i)
            }
            println!(
                "  Properties: {:?}",
                platform.get_properties().unwrap_or_default()
            );
            println!("----\n\n");
        }

        Ok(())
    }

    #[test]
    fn cl_command_queue_test() -> Result<(), ClError> {
        let platform = ClPlatform::default().unwrap();
        let devices = platform.get_cpu_devices().unwrap();
        let cpu = devices[0].clone();
        let sub_devices = cpu.create_subdevice_equally(4).unwrap();
        let context = ClContext::new(&sub_devices).unwrap();
        let command_queue_parameters = CommandQueueProperties::<Version20>::new();

        let mut command_queue: Vec<ClCommandQueue> = Vec::new();

        let properties = command_queue_parameters.get_properties();
        for device in &sub_devices {
            let queue = ClCommandQueue::create_command_queue_with_properties(
                &context,
                &device,
                &properties,
            )?;
            command_queue.push(queue);
        }

        for queue in command_queue {
            println!("Queue X");
            println!("  Context: {}", queue.get_context().unwrap());
            println!("  Device: {}", queue.get_device().unwrap());
            println!(
                "      Device refence count: {}",
                queue.get_device().unwrap().get_reference_count().unwrap()
            );
            println!(
                "  Reference Count: {}",
                queue.get_reference_count().unwrap()
            );
            println!(
                "  Queue Size: {}",
                queue.get_queue_size().unwrap_or_default()
            )
        }

        Ok(())
    }

    #[test]
    fn program_test() -> Result<(), ClError> {
        let platform = ClPlatform::default()?;
        let devices_list = platform.get_all_devices()?;
        let device = devices_list.first().unwrap();
        let subdevice = device.create_subdevice_equally(4)?;
        let context = ClContext::new(&subdevice)?;
        let program_source = std::fs::read_to_string("./tests/program1test/add.cl").unwrap();
        let unbuilded_program = ClProgram::<NotBuilded>::from_src(&context, program_source)?;

        println!("Source: {}", unbuilded_program.get_source()?);
        println!("Num Devices: {}", unbuilded_program.get_num_devices()?);

        let build_options = ProgramParameters::default();
        let build_options = build_options.get_parameters();
        let builded_program = unbuilded_program.build(&build_options, &subdevice)?;
        Ok(())
    }

    #[tokio::test]
    async fn launch_kernel_without_svm() -> Result<(), ClError> {
        // --- Plataforma y dispositivos ---
        let platform = ClPlatform::default()?;
        let devices_list = platform.get_all_devices()?;
        let device = devices_list.first().unwrap();
        let device = device.create_subdevice_equally(4)?;
        let context = ClContext::new(&device)?;

        // --- Programa OpenCL ---
        let program_source = std::fs::read_to_string("./tests/program1test/add.cl").unwrap();
        let unbuilded_program = ClProgram::<NotBuilded>::from_src(&context, program_source)?;

        println!("Source: {}", unbuilded_program.get_source()?);
        println!("Num Devices: {}", unbuilded_program.get_num_devices()?);

        let build_options = ProgramParameters::default().get_parameters();
        let builded_program = match unbuilded_program.build(&build_options, &device) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Program build error:");
                for dev in &device {
                    let log = unbuilded_program.get_logs(dev)?;
                    eprintln!("Device log:\n{}", log);
                }
                panic!();
            }
        };

        // --- Kernel ---
        let kernel = ClKernel::new(&builded_program, "add")?;

        // --- Command queue ---
        let queue_properties = CommandQueueProperties::<Version20>::new()
            .set_cl_queue_properties(true, true, false, false);
        let queue = ClCommandQueue::create_command_queue_with_properties(
            &context,
            &device.first().unwrap(),
            &queue_properties.get_properties(),
        )?;

        // --- Buffers y datos ---
        let size = 2048;
        let mut sum_a: Vec<f32> = vec![1.0; size];
        let mut sum_b: Vec<f32> = vec![2.0; size];

        let buffer_a_flags = vec![MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr];
        let buffer_b_flags = vec![MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr];

        let buffer_a = ClBuffer::new(
            &context,
            &buffer_a_flags,
            size * size_of::<f32>(),
            sum_a.as_mut_ptr() as *mut c_void,
        )?;

        let buffer_b = ClBuffer::new(
            &context,
            &buffer_b_flags,
            size * size_of::<f32>(),
            sum_b.as_mut_ptr() as *mut c_void,
        )?;

        // --- Setear argumentos del kernel ---
        unsafe {
            kernel.setArgs(0, size_of::<*mut c_void>(), buffer_a.as_ptr())?;
            kernel.setArgs(1, size_of::<*mut c_void>(), buffer_b.as_ptr())?;
        }

        // --- Ejecutar kernel ---
        let event = queue
            .enqueue_nd_range_kernel(&kernel, 1, vec![0], vec![size], vec![64], None, None)
            .await?;

        // --- Leer resultados ---
        queue
            .enqueue_read_buffer(&buffer_a, None, &mut sum_a, None)
            .await?;

        println!("Primeros 10 resultados: {:?}", &sum_a[..10]);

        Ok(())
    }

    #[tokio::test]
    async fn launch_kernel_with_svm() -> Result<(), ClError> {
        // --- Plataforma y dispositivos ---
        let platforms = ClPlatform::get_all()?;
        let binding = ClPlatform::default()?;
        let platform = platforms.first().unwrap_or(&binding);
        let devices_list = platform.get_all_devices()?;
        let device = devices_list.first().unwrap();
        println!("Device capabilities: {}", device.get_svm_capabilities()?);

        let context = ClContext::new(&devices_list)?;

        // --- Programa OpenCL ---
        let program_source = std::fs::read_to_string("./tests/program2testsvm/add.cl").unwrap();
        let unbuilded_program = ClProgram::<NotBuilded>::from_src(&context, program_source)?;

        println!("Source: {}", unbuilded_program.get_source()?);
        println!("Num Devices: {}", unbuilded_program.get_num_devices()?);

        let build_options = ProgramParameters::new().version("CL2.0").get_parameters();
        let builded_program = match unbuilded_program.build(&build_options, &devices_list) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Program build error:");
                for dev in &devices_list {
                    let log = unbuilded_program.get_logs(dev)?;
                    eprintln!("Device log:\n{}", log);
                }
                panic!();
            }
        };

        // --- Kernel ---
        let kernel = ClKernel::new(&builded_program, "add")?;

        // --- Command queue ---
        let queue_properties = CommandQueueProperties::<Version20>::new()
            .set_cl_queue_properties(true, true, false, false);
        let queue = ClCommandQueue::create_command_queue_with_properties(
            &context,
            &device,
            &queue_properties.get_properties(),
        )?;

        // --- Buffers y datos ---
        let data_size = 1024;
        let byte_size_of = size_of::<f32>();
        let mut svm_a = ClSvmBuffer::<f32>::new(
            &context,
            &vec![MemoryFlags::ReadWrite],
            data_size,
            0,
        )?;

        let mut svm_b = ClSvmBuffer::<f32>::new(
            &context,
            &vec![MemoryFlags::ReadWrite],
            data_size,
            0,
        )?;

        {
            let mut svm_a_lock = svm_a.map_mut(&queue, &vec![MemoryFlags::WriteOnly])?;
            let mut svm_b_lock = svm_b.map_mut(&queue, &vec![MemoryFlags::WriteOnly])?;
            for i in 0..data_size {
                svm_a_lock[i] = i as f32;
                svm_b_lock[i] = 1.0;
            }
        }
        // --- Setear argumentos del kernel ---
        unsafe {
            kernel.setSvmArg(0, svm_a.len, svm_a.as_ptr())?;
            kernel.setSvmArg(1, svm_b.len, svm_b.as_ptr())?;
        }

        // --- Ejecutar kernel ---
        let event = queue
            .enqueue_nd_range_kernel(&kernel, 1, vec![0], vec![data_size], vec![64], None, None)
            .await?;

        // --- Leer resultados ---
        {
            let svm_a_lock = svm_a.map_mut(&queue, &vec![MemoryFlags::ReadOnly])?;
            println!("Primeros 10 resultados: {:?}", &svm_a_lock[..10]);
        }
        

        Ok(())
    }
}
