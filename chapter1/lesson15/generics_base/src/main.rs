// Без обобщений — дублирование кода
fn max_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
fn max_i64(a: i64, b: i64) -> i64 { if a > b { a } else { b } }

// С обобщениями — одна реализация для всех типов
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
// T: PartialOrd — трейт-граница: T должен реализовывать трейт PartialOrd (поддерживать >)
// Без границы T: PartialOrd сравнение a > b не скомпилируется 
// — компилятор не знает, как сравнивать произвольный тип T.

// Generic struct: хранит значение любого типа
pub struct Container<T> {
    pub value: T,
    pub label: String,
}

// Конструктор с обобщением
impl<T> Container<T> {
    pub fn new(value: T, label: &str) -> Self {
        Container {
            value,
            label: label.to_string(),
        }
    }
    
    // Метод, возвращающий ссылку на внутреннее значение
    pub fn get<'a>(&'a self) -> &'a T {
        &self.value
    }
}

// Пара значений разных типов
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

// Метод, работающий только если T реализует Display (для вывода)
use std::fmt::Display;
impl<T: Display, U> Pair<T, U> {
    pub fn print_first(&self) {
        println!("First: {}", self.first);
    }
}

// Одна граница: T должен реализовывать Debug
fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

// Несколько границ: T должен реализовывать и Debug, и Clone
fn clone_and_print<T: std::fmt::Debug + Clone>(value: T) {
    let cloned = value.clone();
    println!("{:?}", cloned);
}

// Граница на метод: вызов метода трейта внутри функции
use crate::resource::TempResource;  // из Занятия 14
fn log_resource<T: TempResource>(res: T) {
    println!("Resource: {}, active: {}", res.id(), res.is_active());
}


fn main() {
    let c1 = Container::new(42, "number");      // T = i32
    let c2 = Container::new("hello", "text");   // T = &str
    let c3 = Container::new(FileDescriptor::new("fd1", 10, 0), "resource"); // T = FileDescriptor
    println!("Hello, world!");
}