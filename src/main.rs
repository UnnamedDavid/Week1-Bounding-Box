use core::str;
use std::string;

fn main() {
    println!("{}hello rust!", hello_world());
}

fn hello_world() -> String {
    String::from("Hello world, ")
}
