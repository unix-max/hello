use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

struct Guess {
	value: i32,
}
impl Guess {
	fn new(val:i32) -> Self {
		if val<1 || val >100 {
			panic!("value must be < 100 ");
		} else {
			Self {value: val}
		}
	}
	pub fn value(&self) -> i32 {
        self.value
	}
}
fn main() {
    println!("Игра в загадки");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Введите число");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения строки");

        let guess:Guess = match guess.trim().parse() {
            // Ok(num) => num,
						Ok(num) => Guess::new(num),
            Err(_) => continue,
        };
        // println!("Ваше число {guess}");
				// if guess<1 || guess >100 {
				// 	println!("Ваше число должно быть в диаппазоне от 1 до 100");
				// }

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Мало"),
            Ordering::Greater => println!("Много"),
            Ordering::Equal => {
                println!("Попал");
                break;
            },
        }
    }


}
