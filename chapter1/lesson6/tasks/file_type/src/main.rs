enum FileType {
    Text(u32),
    Image { width: u32, height: u32 },
    Audio(u32),
}

impl FileType {
    fn get_description(&self) -> String {
        match self {
            FileType::Text(size) => format!("Текстовый файл, {} байт", size),
            FileType::Image { width, height } => format!("Изображение, разрешение {}x{} пикселей", width, height),
            FileType::Audio(duration_sec) => format!("Аудио файл, длительность {} секунд", duration_sec),
        }
    }
}

fn main() {
    let text_file = FileType::Text(1024);
    let image_file = FileType::Image { width: 1980, height: 1080 };
    let audio_file = FileType::Audio(1488);

    println!("{}", text_file.get_description()); // "Текстовый файл, 1024 байт"
    println!("{}", image_file.get_description()); // "Изображение, разрешение 1980x1080 пикселей"
    println!("{}", audio_file.get_description()); // "Аудио файл, длительность 1488 секунд"
}