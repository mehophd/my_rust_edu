use crate::plugin::*;

pub struct PluginRunner {
    logger: LoggerPlugin,
    validator: ValidatorPlugin
}

impl PluginRunner {
    pub fn new(logger: LoggerPlugin, validator: ValidatorPlugin) -> Self {
        PluginRunner {
            logger: logger,
            validator: validator
        }
    }

    pub fn run_pipeline(&self, input: &str) -> Result<String, String> {        
        match self.validator.execute(input) {
            Ok(result) => {
                match self.logger.execute(&result) {
                    Ok(fin) => Ok(fin),
                    Err(err2) => Err(err2)  
                }
            },
            Err(err1) => Err(err1)
        }
    }

    pub fn list_plugins<'a>(&'a self) -> Vec<(&'a str, u32)> {
        let mut result: Vec<(&str, u32)> = Vec::new();
        result.push((self.logger.name(), self.logger.version()));
        result.push((self.validator.name(), self.validator.version()));
        result
    }
}