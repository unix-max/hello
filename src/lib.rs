pub mod garden;
use crate::garden::vegetables::Asparagus;
pub fn hello() -> Asparagus {
    println!("Hello");
    let asp = Asparagus {
        name: String::from("Maru"),
        age: 12,
    };
    let rouse1 = flower::Rouse {
        list: 10,
        color:(255,255,0),
    };
    asp
}
pub mod flower {
    pub struct Rouse {
        pub list: u8,
        pub color: (u8, u8,u8),
    }
}