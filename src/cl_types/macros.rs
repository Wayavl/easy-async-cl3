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
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::platform::get_platform_data(self.value, $value_id)
                    .map_err(crate::error::cl_platform::PlatformError::GetDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::cl_platform::PlatformError::FailedToFormat)
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
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::device::get_device_data(self.value, $value_id)
                    .map_err(crate::error::cl_device::DeviceError::GetDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::cl_device::DeviceError::FailedToFormat)
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_context_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::cl_context::ContextError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::context::get_context_data(self.value, $value_id)
                    .map_err(crate::error::cl_context::ContextError::GetDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::cl_context::ContextError::FailedToFormat)
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_command_queue_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::cl_command_queue::CommandQueueError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::command_queue::get_command_queue_data(self.value, $value_id)
                    .map_err(crate::error::cl_command_queue::CommandQueueError::QueryDataError)?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::cl_command_queue::CommandQueueError::FailedToFormat)
            }
        )*
    };
}