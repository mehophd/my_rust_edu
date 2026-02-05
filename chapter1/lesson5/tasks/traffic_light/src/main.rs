struct TrafficLight {
    state: String,
    timer: u32,
}

impl TrafficLight {
    fn new() -> TrafficLight {
        TrafficLight {
            state: String::from("red"),
            timer: 30
        }
    }

    fn next(&mut self) {
        if self.state == "red" {
            self.state = String::from("green");
            self.timer = 60;
        } else if self.state == "green" {
            self.state = String::from("yellow");
            self.timer = 5;
        } else {
            self.state = String::from("red");
            self.timer = 30;
        }
    }

    fn tick(&mut self) {
        self.timer -= 1; // не хватает проверки, если timer уже ноль
        if self.timer == 0 {
            self.next();
        }
    }

    fn get_info(&self) -> String {
        format!("Светофор: {}, осталось: {}", self.state, self.timer)
    }

    fn emergency(&mut self) {
        self.state = String::from("red");
        self.timer = 30;
    }
}


fn main() {
    let mut trafic = TrafficLight::new();
    println!("{}", trafic.get_info());

    trafic.tick();
    println!("{}", trafic.get_info());

    while trafic.state != "yellow" {
        trafic.tick();
        println!("{}", trafic.get_info());
    }
}