pub mod garden;
use crate::garden::vegetables::Asparagus;
pub fn hello() -> Asparagus {
    println!("Hello");
    let asp = Asparagus {
        name: String::from("Maru"),
        age: 12,
    };
    asp
}