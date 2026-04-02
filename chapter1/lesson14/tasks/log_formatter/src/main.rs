use std::fmt;

#[derive(Debug)]
enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR
}

trait Logger {
    fn log(&mut self, level: LogLevel, message: &str) -> Result<(), String>;
    fn flush(&mut self) -> Result<(), String>;
}

struct LogEntry {
    message: String,
    level: LogLevel
}

struct ConsoleLogger {
    message: String,
    level: LogLevel
}

struct BufferLogger {
    buffer: Vec<LogEntry>,
}

impl Logger for ConsoleLogger {
    fn log(&mut self, level: LogLevel, message: &str) -> Result<(), String> {
        if message.chars().count() == 0 {
            return Err("Сообщение не может быть пустым".to_string());
        }
        self.message = message.to_string();
        self.level = level;

        println!("{:?} {}", self.level, &self.message);

        return Ok(());
    }

    fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }
}

impl fmt::Debug for BufferLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = f.debug_list();
        for entry in &self.buffer {
            let message = format!("[{:?}] {}", entry.level, entry.message);
            result.entry(&message);
        }
        result.finish()
    }
}

impl Logger for BufferLogger {
    fn log(&mut self, level: LogLevel, message: &str) -> Result<(), String> {
        if message.chars().count() == 0 {
            return Err("Сообщение не может быть пустым".to_string());
        }

        if self.buffer.len() >=  100 {
            return Err("Буфер переполнен".to_string());
        }

        let entry = LogEntry { message: message.to_string(), level: level};

        self.buffer.push(entry);

        return Ok(());
    }

    fn flush(&mut self) -> Result<(), String> {
        println!("{:?}", self);
        self.buffer.clear();
        Ok(())
    }
}

fn main() {
    let mut logger1 = ConsoleLogger { message: String::new(), level: LogLevel::DEBUG};
    let mut logger2 = BufferLogger { buffer: Vec::new() };

    match logger1.log(LogLevel::INFO, "hello") {
        Ok(()) => {},
        Err(mes) => println!("{}", mes),
    }

    match logger1.log(LogLevel::INFO, "") {
        Ok(()) => {},
        Err(mes) => println!("{}", mes),
    }

    match logger2.log(LogLevel::DEBUG, "hello") {
        Ok(()) => {},
        Err(mes) => println!("{}", mes),
    }

    match logger2.log(LogLevel::DEBUG, "") {
        Ok(()) => {},
        Err(mes) => println!("{}", mes),
    }

    logger2.log(LogLevel::DEBUG, "world");
    logger2.flush();
    logger2.flush();

    for i in 0..101 {
        match logger2.log(LogLevel::ERROR, "trash") {
            Ok(()) => {},
            Err(mes) => println!("{}", mes),
        }
    }

    // INFO hello
    // Сообщение не может быть пустым
    // Сообщение не может быть пустым
    // ["[DEBUG] hello", "[DEBUG] world"]
    // []
    // Буфер переполнен
}
