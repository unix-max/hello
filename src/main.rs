
fn main() {
	let s1 = String::from("hi");
	//let s2= s1[1];
	{

		let hello = String::from("Hola");
		let hello = String::from("Привет");
		//let answer = &hello[0];
		let s =&hello[0..4];
		println!("s {s}");

	}
	{
		let hello = "Привет";
		for i in &mut hello.chars() {
			println!("char {i}");
		}
		for i in &mut hello.bytes() {
			println!("B {i}");
		}
	}
	println!("Count End");
	
}
