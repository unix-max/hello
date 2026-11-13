#[derive(Debug)]
struct Restangle {
	width: u32,
	height: u32
}
fn main() {
let conv = vec![|x: i32| -> i32 { x + 1 }, |x| x+2, |x| x+3];
for (index , clos) in conv.iter().enumerate() {
	let a = clos(10+ index as i32);
	println!("It`s a {a}");
}
}




