use std::marker::PhantomData;

pub struct Version20;
pub struct Version10;
pub struct CommandQueueProperties<T> {
    properties: Vec<u64>,
    old_params: u64,
    phantom_value: PhantomData<T>
}

impl CommandQueueProperties<Version20> {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            old_params: 0,
            phantom_value: PhantomData
        }
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn set_cl_queue_properties(mut self, cl_queue_out_of_order_exec_mode_enable: bool, cl_queue_profiling_enable: bool, cl_queue_on_device: bool, cl_queue_on_device_default: bool) -> Self {
        let key = cl3::command_queue::CL_QUEUE_PROPERTIES as cl3::types::cl_command_queue_properties;
        let queue_on_device = cl3::command_queue::CL_QUEUE_ON_DEVICE as cl3::types::cl_command_queue_properties;
        let queue_on_device_default = cl3::command_queue::CL_QUEUE_ON_DEVICE_DEFAULT as cl3::types::cl_command_queue_properties;
        let queue_out_of_order_exec_mode_enable = cl3::command_queue::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE as cl3::types::cl_command_queue_properties;
        let queue_profiling = cl3::command_queue::CL_QUEUE_PROFILING_ENABLE as cl3::types::cl_command_queue_properties;

        let mut properties = 0;

        if cl_queue_out_of_order_exec_mode_enable {
            properties |= queue_out_of_order_exec_mode_enable;
        }
        if cl_queue_profiling_enable {
            properties |= queue_profiling;
        }
        if cl_queue_on_device {
            properties |= queue_on_device | queue_out_of_order_exec_mode_enable;
        }
        if cl_queue_on_device_default {
            properties |= queue_on_device_default | queue_on_device | queue_out_of_order_exec_mode_enable;
        }

        self.properties.push(key);
        self.properties.push(properties);

        self
    }

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn set_cl_queue_size(mut self,size: u32) -> Self {
        self.properties.push(cl3::command_queue::CL_QUEUE_SIZE as cl3::types::cl_command_queue_properties);
        self.properties.push(size as cl3::types::cl_command_queue_properties);
        self
    }

    pub fn get_properties(mut self) -> Vec<u64> {
        self.properties.push(0);
        self.properties
    }
}

impl Default for CommandQueueProperties<Version20> {
    fn default() -> Self {
        Self {
            old_params: 0,
            properties: Vec::new(),
            phantom_value: PhantomData
        }
    }
}

impl CommandQueueProperties<Version10> {
    pub fn new() -> Self {
        Self {
            old_params: 0,
            properties: Vec::new(),
            phantom_value: PhantomData
        } 
    }

    pub fn get_properties(mut self) -> u64 {
        self.old_params
    }

    pub fn cl_queue_out_of_order_exec_mode_enable(mut self) -> Self {
        self.old_params |= cl3::command_queue::CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE;
        self
    }

    pub fn cl_queue_profiling_enable(mut self) -> Self {
        self.old_params |= cl3::command_queue::CL_QUEUE_PROFILING_ENABLE;
        self
    }
}