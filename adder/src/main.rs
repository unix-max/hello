use add_one::{self};
use add_two::{self};
fn main() {
	let num:i32 = 10;
	let a = add_one::add_one(num);
	println!("summ is {a}");
	
	let b = add_two::add(12,5);
	println!("this is b {b}");
}





