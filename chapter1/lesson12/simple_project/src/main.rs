// src/main.rs
mod network;      // подключает src/network.rs
mod utils;        // подключает src/utils/mod.rs

fn main() {
    network::connect();
    let result = utils::math::add(2, 3);

    println!("{}", result);
}