use hello::hello;
use hello::front_of_house::hosting::add_to_waitlist;
use std::collections::HashMap;
use std::io;
use std::fmt;
use std::io::Result as ioResult;
fn main() {
	let a= hello();
	let mut map: HashMap<u32, String> = HashMap::new();
	map.insert(1, String::from("max"));
	println!("Count");
}
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Метод write! возвращает fmt::Result
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn finc2() -> io::Result<String> {

	Ok(String::from("str"))
}
fn finc3() -> ioResult<String> {

	Ok(String::from("str"))
}

