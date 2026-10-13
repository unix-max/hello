
fn main() {
	
	
	{
		let mut s1 = String::from("hello");
		let r1 = &s1;
		let r2 = &s1;
		println!("{r1} {r2} длиной ");

		let mut r3 = &mut s1;
		println!("{r3} длиной ");
	}
}


// fn get_len_chptr(s: &mut String) -> usize {
// 	s.push_str(", world");
// 	s.len()
// }