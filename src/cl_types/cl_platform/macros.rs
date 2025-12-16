#[macro_export]
macro_rules! generate_getters {
    (
        $(
            ($name:ident, $type_of:ty, $value_id:expr)
        ),* $(,)?
    ) => {
        $(
            pub fn $name(&self)
                -> Result<$type_of, $crate::error::cl_platform::PlatformError>
            where
                $type_of: $crate::cl_types::formater::Formater,
            {
                let buffer =
                    cl3::platform::get_platform_data(self.value, $value_id).map_err(crate::error::cl_platform::PlatformError::FormatError)?;

                Ok(<$type_of as $crate::cl_types::formater::Formater>::from_buffer(buffer))
            }
        )*
    };
}

