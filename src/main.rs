fn anther_fn(x:i32, unit_label:char) -> i32{
	println!("This is another fn {} {}", x+1, unit_label);
	return x+12;
}
fn my_fn()  -> fn(i32, char) -> i32{
	 anther_fn
}
fn main() {
	println!("This is main");
	let z = anther_fn(5, 'q');
	let y= {
		let a = 10;
		a+12
	};
	println!("This is Y {y} and {z}");
	let a_fn=my_fn();
	let bx = a_fn(10, 's');
	println!("This is bx {bx}");
 

}
