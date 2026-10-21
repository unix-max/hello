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

pub mod front_of_house {
    pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_restarant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    
}