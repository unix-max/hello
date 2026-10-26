
use std::fs::{ self, File } ;
use std::io::{self, Read, ErrorKind};
fn main() {
	let fc = File::open("hello.txt")
		.expect("hello.txt should be included in this project");
	//let fc = File::open("hello.txt")?;
}

fn open_file() {
	 let greeting_file_result = File::open("hello.txt");
		let greeting_file = match greeting_file_result {
			Ok(file) => file,
			Err(error) => match error.kind() {
				ErrorKind::NotFound => match File::create("hello.txt") {
					Ok(fc) => fc,
					Err(e) => panic!("error create file {e:?}"),
				}
				_ => panic!("Problem opening the file: {error:?}"),
			},
		};
}


fn read_user_from_file() -> Result<String, io::Error> {
	let username_file_result = File::open("hello.txt");
	let mut urername_file = match username_file_result {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	let mut username = String::new();
	match urername_file.read_to_string(&mut username) {
		Ok(_) => Ok(username),
		Err(e) => Err(e)
	}
}

fn read_user_from_file1() -> Result<String, io::Error> {
	let mut username_file = File::open("hello.txt")?;
	let mut username = String::new();
	username_file.read_to_string(&mut username)?;
	Ok(username)
}

fn read_user_from_file2() -> Result<String, io::Error> {
	let mut username = String::new();
	File::open("hello.txt")?.read_to_string(&mut username)?;
	Ok(username)
}

fn read_user_from_file3() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()

}
