pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> u32;
    fn execute(&self, input: &str) -> Result<String, String>;
    fn priority(&self) -> u8 { 0 }  // метод по умолчанию
}

pub struct LoggerPlugin {
    name: String,
    version: u32,
    log_prefix: String
}

impl LoggerPlugin {
    pub fn new(name: &str, version: u32, log_prefix: &str) -> Self {
        LoggerPlugin {
            name: name.to_string(),
            version: version,
            log_prefix: log_prefix.to_string()
        }
    }
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("{}: {}", self.log_prefix, input))
    }
    fn priority(&self) -> u8 { 
        10 
    }
}

pub struct ValidatorPlugin {
    name: String,
    version: u32,
    min_length: usize
}

impl ValidatorPlugin {
    pub fn new(name: &str, version: u32, min_length: usize) -> Self {
        ValidatorPlugin {
            name: name.to_string(),
            version: version,
            min_length: min_length
        }
    }
}

impl Plugin for ValidatorPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        if input.len() >= self.min_length {
            Ok(input.to_string())
        } else {
            Err("Input too short".to_string())
        }
    }
    fn priority(&self) -> u8 { 
        0
    }
}