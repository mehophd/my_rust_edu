pub struct Logger {
    prefix: String,
    min_level: LogLevel,
    enabled: bool
}

pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LogLevel {
    pub fn is_equal(pat: &LogLevel, other: &LogLevel) -> bool {
        match (pat, other) {
            (LogLevel::DEBUG, LogLevel::DEBUG) => true,
            (LogLevel::INFO, LogLevel::INFO) => true,
            (LogLevel::WARN, LogLevel::WARN) => true,
            (LogLevel::ERROR, LogLevel::ERROR) => true,
            _ => false,
        }
    }

    pub fn get_level_string(level: &LogLevel) -> String {
            match level {
                LogLevel::DEBUG => String::from("DEBUG"),
                LogLevel::INFO => String::from("INFO"),
                LogLevel::WARN => String::from("WARN"),
                LogLevel::ERROR => String::from("ERROR"),
            }
        }
}

impl Logger {
    pub fn new(prefix: String, min_level: LogLevel) -> Logger {
        Logger {
            prefix: prefix,
            min_level: min_level,
            enabled: true
        }
    }

    fn check_priority(&self, level: &LogLevel) -> bool {
        match level {
            LogLevel::DEBUG => LogLevel::is_equal(&self.min_level, &LogLevel::DEBUG),
            LogLevel::INFO => LogLevel::is_equal(&self.min_level, &LogLevel::DEBUG) || LogLevel::is_equal(&self.min_level, &LogLevel::INFO),
            LogLevel::WARN => !LogLevel::is_equal(&self.min_level, &LogLevel::ERROR),
            LogLevel::ERROR => true,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::DEBUG, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::INFO, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::WARN, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::ERROR, msg);
    }

    fn log(&self, level:LogLevel, msg: &str) -> String {
        if self.is_enabled() && self.check_priority(&level) {
            println!("[{}][{}] {}", self.prefix, LogLevel::get_level_string(&level), msg);
            return String::from(msg);
        }
        return String::new();
    }
}