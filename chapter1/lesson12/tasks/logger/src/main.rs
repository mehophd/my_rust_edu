mod logger;

use logger::{Logger, LogLevel};

fn main() {
    let mut app_logger = Logger::new(String::from("APP"), LogLevel::INFO);
    let mut kernel_logger = Logger::new(String::from("KERNEL"), LogLevel::WARN);

    app_logger.debug("Это не должно напечататься (DEBUG < INFO)");
    app_logger.info("Приложение запущено");
    app_logger.error("Критическая ошибка в приложении");

    kernel_logger.info("Это не должно напечататься (INFO < WARN)");
    kernel_logger.warn("Низкий заряд батареи");
    kernel_logger.error("Отказ системы");

    app_logger.disable();
    if !app_logger.is_enabled() {
        println!("Логгер APP успешно выключен.");
    }

    app_logger.error("Это сообщение не появится, так как логгер выключен");
    
    app_logger.enable();
    app_logger.info("Логгер снова включен и это сообщение видно");

    // Вывод:
    // [APP][INFO] Приложение запущено
    // [APP][ERROR] Критическая ошибка в приложении
    // [KERNEL][WARN] Низкий заряд батареи
    // [KERNEL][ERROR] Отказ системы
    // Логгер APP успешно выключен.
    // [APP][INFO] Логгер снова включен и это сообщение видно
}