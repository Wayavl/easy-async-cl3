#[macro_export]
macro_rules! cl_platform_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::cl_platform::PlatformError>
            where
                $type_of: $crate::cl_types::formater::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::platform::get_platform_data(self.value, $value_id)
                    .map_err(crate::error::cl_platform::PlatformError::GetDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                Ok(<$type_of as $crate::cl_types::formater::Formatter>::from_buffer(&buffer).unwrap_or_default())
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_device_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::cl_device::DeviceError>
            where
                $type_of: $crate::cl_types::formater::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::device::get_device_data(self.value, $value_id)
                    .map_err(crate::error::cl_device::DeviceError::GetDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                Ok(<$type_of as $crate::cl_types::formater::Formatter>::from_buffer(&buffer).unwrap_or_default())
            }
        )*
    };
}
