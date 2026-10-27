
fn main() {
	let str1 = String::from("abcde");
	let long: &str;
	let long1: &str;
	{
	let str2 = String::from("xyz");
	long = longest(&str1, &str2);
	long1 = longest1(&str1, &str2);
	println!("Longnest str {long}");
	}
	println!("Longnest str {long1}");
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
	if a.len() > b.len() { a } else { b }
}
fn longest1<'a>(a: &'a str, b: & str) -> &'a str {
	 a
}
fn longest2<'a>(a: &'a str, b: & str) -> &'a str {
	let z =String::from("Hello");
	z.as_str()
}