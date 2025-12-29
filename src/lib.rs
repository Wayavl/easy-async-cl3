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
    use crate::{
        cl_types::{
            cl_command_queue::{
                ClCommandQueue,
                command_queue_parameters::{CommandQueueProperties, Version10, Version20},
            },
            cl_context::ClContext,
            cl_device::{device_type::ALL},
            cl_platform::ClPlatform,
        }, error::ClError,
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
        let devices= platform.get_cpu_devices().unwrap();
        let cpu = devices[0].clone();
        let sub_devices = cpu.create_subdevice_equally(4).unwrap();
        let context = ClContext::new(&sub_devices).unwrap();
        let command_queue_parameters = CommandQueueProperties::<Version20>::new();

        let mut command_queue: Vec<ClCommandQueue> = Vec::new();


        let properties = command_queue_parameters.get_properties();
        for device in &sub_devices {
            let queue = ClCommandQueue::create_command_queue_with_properties(&context, &device, &properties)?;
            command_queue.push(queue);
        }

        for queue in  command_queue {
            println!("Queue X");
            println!("  Context: {}", queue.get_context().unwrap());
            println!("  Device: {}", queue.get_device().unwrap());
            println!("      Device refence count: {}", queue.get_device().unwrap().get_reference_count().unwrap());
            println!("  Reference Count: {}", queue.get_reference_count().unwrap());
            println!("  Queue Size: {}", queue.get_queue_size().unwrap_or_default())
        }

        Ok(())
    }

    #[test]
    fn program_test() -> Result<(), ClError> {
        let platform  = ClPlatform::default()?;
        let devices_list = platform.get_all_devices()?;
        let device = devices_list.first().unwrap();
        let subdevice = device.create_subdevice_equally(4)?;
        let context = ClContext::new(&subdevice)?;

        Ok(())
    }
}
