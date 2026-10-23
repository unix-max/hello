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
pub(crate) mod flower {
    pub struct Rouse {
        pub list: u8,
        pub color: (u8, u8,u8),
    }
}
pub fn add_to_waitlist() {}
fn delivery_order() {

}

pub mod front_of_house {
    fn private_funct() {
        println!("Hello from private house");
        hosting::seat_at_table();
        crate::add_to_waitlist();
        self::hosting::add_to_waitlist();
    }
    

    pub mod hosting {
       pub fn add_to_waitlist() {}

       pub fn seat_at_table() {
            super::private_funct();
        }
    }

    pub(super) mod serving {
        pub(self) fn take_order() {}

        pub fn serve_order() {
            println!("serve_order");
        }

        fn take_payment() {}
    }
    fn grome() {
        serving::serve_order();
    }
}
mod back_office {
    pub struct Breakfast{
      pub toast: String,
			seasonal_fruit: String,
    }
		impl Breakfast {
			pub fn summer(toast: &str) -> Breakfast {
				Breakfast {
					toast: String::from(toast),
					seasonal_fruit: String::from("peaches")
				}
			}
		}
		pub enum Appetizer {
			Soup,
			Salad,
		}
    fn fix_incorrect_order() {
        cook_order();
        super::delivery_order();
    }
    fn cook_order() {}

}

pub fn eat_at_restarant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
  
}
pub fn eat_at_restarant1(){
	let mut meal = back_office::Breakfast::summer("Ray");
	meal.toast = String::from("Wheat");
	print!("I like`d {} toast please", meal.toast);
	//meal.seasonal_fruit = String::from("blueberries");
	let order1 = back_office::Appetizer::Salad;
	let order2 = back_office::Appetizer::Soup;
}

use crate::front_of_house::serving;

mod customer{
	use crate::front_of_house::hosting;
	pub fn eat_at_restarant2() {
		hosting::add_to_waitlist();
		super::serving::serve_order();
	}
}
