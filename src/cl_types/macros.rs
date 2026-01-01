#[macro_export]
macro_rules! cl_platform_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::platform::get_platform_data(self.value, $value_id)
                    .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
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
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::device::get_device_data(self.value, $value_id)
                    .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
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
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::context::get_context_data(self.value, $value_id)
                    .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer).ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
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
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::command_queue::get_command_queue_data(self.value, $value_id)
                    .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_program_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::program::get_program_data(self.value, $value_id)
                    .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_program_build_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self, device: &crate::cl_types::cl_device::ClDevice)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                // Obtiene el buffer desde OpenCL
                let buffer = cl3::program::get_program_build_data(self.value, device.as_ptr(), $value_id)
                .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}


#[macro_export]
macro_rules! cl_kernel_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                let buffer = cl3::kernel::get_kernel_data(self.value, $value_id)
                .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_kernel_workgroup_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self, device: crate::cl_types::cl_device::ClDevice)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                let buffer = cl3::kernel::get_kernel_work_group_data(self.value, device.as_ptr(), $value_id)
                .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}

#[macro_export]
macro_rules! cl_kernel_subgroup_generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self, device: crate::cl_types::cl_device::ClDevice)
                -> Result<$type_of, $crate::error::ClError>
            where
                $type_of: $crate::cl_types::formatter::Formatter,
            {
                let buffer = cl3::kernel::get_kernel_work_group_data(self.value, device.as_ptr(), $value_id)
                .map_err(|code| crate::error::ClError::Api(crate::error::api_error::ApiError::get_error(code)))?;

                // Usa from_buffer que ahora devuelve Option<Self>
                <$type_of as $crate::cl_types::formatter::Formatter>::from_buffer(&buffer)
                    .ok_or_else(|| crate::error::ClError::Wrapper(crate::error::wrapper_error::WrapperError::FormatterFailed))
            }
        )*
    };
}

