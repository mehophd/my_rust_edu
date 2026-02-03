struct Point {
    x: i32,
    y: i32,
}

fn distance_from_origin(point: &Point) -> f64 {
    let x = point.x;
    let y = point.y;

    ((x*x + y*y) as f64).sqrt()
}

fn main() {
    let point = Point { x: 3, y: 4};
    println!("Точка: ({}, {})\nРасстояние от начала координат: {}", point.x, point.y, distance_from_origin(&point));
}
