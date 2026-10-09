use std::io;
use rand::prelude::*;
fn main() {
    println!("Игра в загадки");
    println!("Введите число");
    let mut guess = String::new();
    let secret_number = rand::rng().random_range(1..=100);
    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения строки");
    println!("Ваше число {guess}");


}
