
fn main() {
	{
		let mut s = String::from("hello");
		s.push_str(", world");
		let s2 = s.clone();
		println!("s ={s}, s2 ={s2}");
	}
	{
		let x = 5;
		let y = x;
		println!("x ={x}, y ={y}");
	}
	{
		let s = String::from("hello");
		let z = take_owenerchip(s);
		println!("This is z colled z {z}");
		
		let x = 5;
		let y = x;
		let z = make_copy(x);
		println!("This is x {x}, y {y}, z {z}")
	}
	{
		let s = String::from("hello");
		let (s, len) = get_len(s);
		println!("Строка {s} длиной {len}");
	}
	{
		let s1 = String::from("hello");
		let len = get_len_ptr(&s1);
		println!("Строка {s1} длиной {len}");
	}
	{
		let mut s1 = String::from("hello");
		let len = get_len_chptr(&mut s1);
		println!("Строка {s1} длиной {len}");
	}
}
fn take_owenerchip(s: String) -> String {
	println!("This is s {s}");
	s
}

fn make_copy(some_integer: i32) -> i32 {
	println!("this is copy some_integer {some_integer}");
	some_integer
}

fn get_len(s: String) -> (String, usize) {
	let len:usize = s.len();
	(s, len)
}

fn get_len_ptr(s: &String) -> usize {
	s.len()
}

fn get_len_chptr(s: &mut String) -> usize {
	s.push_str(", world");
	s.len()
}