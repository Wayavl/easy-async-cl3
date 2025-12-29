pub struct ProgramParameters {
    parameters: String,
}

impl ProgramParameters {
    pub fn get_parameters(&mut self) -> String {
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
}

impl Default for ProgramParameters {
    fn default() -> Self {
        Self {
            parameters: String::new()
        }
    }
}