struct AppConfig {
    port: u16,
    host: String
}

fn parse_config(port_str: &str, host: &str) -> Result<AppConfig, String> {
    if host == "" {
        return Err("Хост не может быть пустым".to_string()); // преобразует &str в String, удобно тож бывает
    }

    match port_str.parse::<u16>() {
        Ok(port) => {
            if port < 1 || port > 65535 {
                return Err("Порт должен быть в диапазоне 1-65535".to_string());
            }

            Ok(AppConfig {
                port,
                host: host.to_string(),
            })
        }

        Err(_) => {
            // Не получилось распарсить
            Err(format!("Некорректный порт: {}", port_str))
        }
    }
}

fn main() {
    let config = parse_config("8080", "localhost");
    match config {
        Ok(cfg) => println!("Хост: {}, порт: {}", cfg.host, cfg.port),
        Err(e) => println!("Ошибка: {}", e),
    }
}
