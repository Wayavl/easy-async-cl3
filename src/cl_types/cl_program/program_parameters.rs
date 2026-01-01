pub struct ProgramParameters {
    parameters: String,
}

impl ProgramParameters {
    pub fn get_parameters(&self) -> String {
        self.parameters.clone()
    }

    pub fn name(mut self, value: &str) -> Self { 
        self.parameters.push_str("-DNAME=");
        self.parameters.push_str(value);
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.parameters.push_str("-cl-std=");
        self.parameters.push_str(version);
        self
    }

    pub fn new() -> Self {
        Self { 
            parameters: String::new()
        }
    }
}

impl Default for ProgramParameters {
    fn default() -> Self {
        Self {
            parameters: String::from("")
        }
    }
}