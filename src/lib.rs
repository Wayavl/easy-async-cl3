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
        cl_types::{cl_context::ClContext, cl_device::device_type::ALL, cl_platform::ClPlatform},
        error::{cl_context::ContextError, cl_device::DeviceError, cl_platform::PlatformError},
    };

    #[test]
    fn cl_platform_test() -> Result<(), PlatformError> {
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
    fn cl_device_test() -> Result<(), DeviceError> {
        let default_platform = ClPlatform::default().unwrap();
        let all_devices = default_platform.get_all_devices().unwrap();

        for device in all_devices {
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
                    "  Profile: {}",
                    device.get_profile().unwrap_or_default()
                );
        }

        Ok(())
    }

    #[test]
    fn cl_sub_device_test() -> Result<(), DeviceError> {
        let default_platform = ClPlatform::default().unwrap();
        let all_devices = default_platform.get_all_devices().unwrap();

        for parent_device in all_devices {
            let subdevice = parent_device.create_subdevice_equally(3)?;
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
    fn cl_context_test() -> Result<(), ContextError> {

        let platform = ClPlatform::default().unwrap();
        let context_from_device_type = ClContext::new_from_device_type(&platform, ALL)?;
        let devices= platform.get_all_devices().unwrap();
        let context_from_default = ClContext::new(&devices)?;

        let context_array = vec![context_from_device_type, context_from_default];

        for platform in context_array {
            
        }
        
        Ok(())
    }
}
