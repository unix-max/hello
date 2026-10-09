use std::io;
use std::cmp::Ordering;
use rand::prelude::*;
fn main() {
    println!("Игра в загадки");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("Введите число");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения строки");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Ваше число {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Мало"),
            Ordering::Greater => println!("Много"),
            Ordering::Equal => {
                println!("Попал");
                break;
            },
        }
    }


}
