pub trait TempResource {
    fn id(&self) -> &str;
    fn is_active(&self) -> bool;
    fn created_at(&self) -> u64;
}

pub struct FileDescriptor {
    id: String,
    created_at: u64,
    timeout_sec: u64,
    is_open: bool
}

impl TempResource for FileDescriptor {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn is_active(&self) -> bool {
        return self.is_open; // не совсем понял, что надо здесь вернуть
    }

    fn created_at(&self) -> u64 {
        return self.created_at;
    }
}

impl FileDescriptor {
    pub fn new(id: &str, timeout_sec: u64, created_at: u64) -> Self {
        FileDescriptor {
            id: id.to_string(),
            timeout_sec: timeout_sec,
            created_at: created_at,
            is_open: true
        }
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        let calculated = u64::MAX - self.created_at;
        println!("[CLEANUP] Resource '{}' dropped. Open: {}. Lifetime used: {} sec.", self.id, self.is_open, calculated);
    } 
}