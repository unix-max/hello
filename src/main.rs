use std::fmt::Display;
struct ImportExp<'a> {
	part: &'a str,
}
impl<'a> ImportExp<'a> {
	fn level(&self) ->i32 {
		3
	}
	fn anonce(&self, anons: &str) -> &str {
		println!("Anonce {anons}");
		&self.part
	}
}
fn main() {
	let novel = String::from("Call me Ismail. Some years ago...");
	let first_sentense = novel.split('.').next().unwrap();
	let i = ImportExp {part: first_sentense};
	let s: &'static str = "I am static string";
}

fn first_world(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}	
		
	}
	&s[..]
}
fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
	where T: Display {
		println!("Anonsment {ann}");
		if x.len() > y.len() { x } else { y }
	}
