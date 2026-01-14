use std::collections::HashMap;

use crate::{cl_types::{cl_command_queue::ClCommandQueue, cl_context::ClContext, cl_device::ClDevice, cl_kernel::ClKernel, cl_platform::ClPlatform, cl_program::{Builded, ClProgram}}, error::ClError};

pub struct AsyncExecutor {
    devices: HashMap<ClPlatform, Vec<ClDevice>>,
    contexts: HashMap<ClPlatform, ClContext>,
    command_queues: HashMap<ClContext, Vec<ClCommandQueue>>,
    program_sources: Vec<Vec<u8>>,
    active_programs: HashMap<ClContext, ClProgram<Builded>>,
    kernels: HashMap<ClContext, ClKernel>
}

impl AsyncExecutor {
    // Todo: I want to create a Vec that holds labels to set buffers in a declarative way.
    // It should also allow the user to use any memory object available.
    pub async fn run_task<T>(&self) -> T {
        todo!()
    }

    // Changes the schedular to a different type of scheduling. For intance: Split between devices
    pub async fn modify_default_scheduler_choice() {
        todo!()
    }


    // Todo: Frees all memory objects however it stops sending jobs until this finish.
    pub async fn free_all_buffers(&mut self) {
        todo!()
    }
    
    // TODO: A list of devices used in the scheduler.
    pub async fn get_devices_in_use(&self) {
        todo!()
    }

    // TODO: Removes a device from the scheduler forcing the restart it 
    pub async fn remove_device(&mut self) {
        todo!()
    }
    pub async fn add_device(&mut self) {
        todo!()
    }

    // Todo: I want it to wait for all proccess to finish and then free all programs and redo the programs.
    pub async fn get_programs(&self) {
        todo!()
    }

    pub async fn add_program_source(&mut self) {
        todo!()
    }
    pub async fn remove_program_source(&mut self) {
        todo!()
    }

    pub async fn add_program_binary(&mut self) {
        todo!()
    }

    pub async fn remove_program_binary(&mut self) {
       todo!() 
    }

    pub async fn add_program_il(&mut self) {

    }

    pub async fn remove_program_il(&mut self) {

    }

    fn default() -> Result<Self, ClError> {
        let mut devices = HashMap::new();

        let platforms= ClPlatform::get_all()?;

        for p in platforms.iter_mut() {
            
        }

        
        Self {
            devices
        }
    }
}

