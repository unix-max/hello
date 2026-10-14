
fn main() {
	let mut s1 = String::from("hello, world");
	let index = first_world(&s1);
	println!("first world {index}");
	s1.clear();

	let s2 =String::from("hello world");
	//let hello = &s2[0..5];
	let hello = &s2[..5];
	//let world = &s2[7..11];
		let world = &s2[7..];
		println!("{hello} {world}");
	let first = first_world1(&s2);
	//s2.clear();
	println!("first world {first}");

	let world = first_world1(&s2[5..]);
	println!("{world}");
	let hello = first_world1(&s2[..7]);
	println!("{hello}");

	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..=3];
	for (index, &val) in slice.iter().enumerate() {
		println!("index {index} val {val}");
	}

}


fn first_world(str: &String) ->usize {
	let bytes= str.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	bytes.len()
}

fn first_world1(s: &str) ->&str {
	let bytes= s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
}