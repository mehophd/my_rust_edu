enum OrderStatus {
    New,
    Processing(u32),
    Shipped { tracking_number: String, shipping_date: String},
    Delivered,
    Cancelled(String),
}

impl OrderStatus {
    fn is_final(&self) -> bool {
        match self {
            OrderStatus::Delivered => true,
            OrderStatus::Cancelled(reason) => true,
            _ => false,
        }
    }

    fn get_info(&self) -> String {
        match self {
            OrderStatus::New => String::from("Письмо создано"),
            OrderStatus::Processing(employee_id) => format!("Письмо в обработке сотрудником {}", employee_id),
            OrderStatus::Shipped {tracking_number, shipping_date} => format!("Письмо {} отправлено {}", tracking_number, shipping_date),
            OrderStatus::Delivered => String::from("Письмо доставлено"),
            OrderStatus::Cancelled(reason) => format!("Письмо отменено по причине \"{}\"", reason),
        }
    }
}

fn main() {
    let order = OrderStatus::Shipped {
        tracking_number: String::from("z1488ov"), 
        shipping_date: String::from("11.11.2011")
    };

    let status = order.is_final();
    if !status { println!("Письмо в процессе"); } else { println!("Процесс завершен"); }
    println!("{}", order.get_info());
}
