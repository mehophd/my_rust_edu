enum UserRole {
    Guest,
    User(u32),
    Moderator { id: u32, permissions: String },
    Admin(u32),
}

struct AccessRequest {
    resource: String,
    action: String,
}

impl UserRole {
    fn can_access(&self, request: &AccessRequest) -> bool {
        match self {
            UserRole::Guest => request.action == String::from("read"),
            UserRole::User(id) => request.action == String::from("read"),
            UserRole::Moderator { id, permissions } => request.action == *permissions, // ошибка в логике
            UserRole::Admin(id) => true,
        }
    }
}

fn main() {
    let user = UserRole::Moderator {
        id: 228,
        permissions: String::from("read,write")
    };

    let access = AccessRequest { 
        resource: String::from("youtube"),
        action: String::from("read,write")
    };

    if user.can_access(&access) {println!("Доступ есть");}
}