fn main() {
  let x = 10;
  println!("your x = {}",x);
	let x = x+ 5;
	{
		let x = 30;
		println!("your x = {}",x);
	}
println!("your x = {}",x);
const THREE_HOURS: i32 = 60* 60 *3;
println!("your 3 Hourse = {}",THREE_HOURS);

let spaces = "       ";
let spaces = spaces.len();
println!("your spaces = {}", spaces);

let r:f32 = 3.0;
println!("your var = {}", r);
let r:u16 = 5/x;
println!("your var = {}", r);
let r:u32 = 12_506;
println!("your var u32 = {}", r);
let r:u16 = 1345;
println!("your var u16 = {}", r);
let r:u8 = 0xfa;
println!("your var u8 = {}", r);
let r:u8 = 0o77;
println!("your var u8 oct = {}", r);
let r:u8 = 0b10100010;
println!("your var u8 bin = {}", r);
let r:u8 = b'A';
println!("your var u8 char = {}", r);
let b:bool = true;
println!("your var bool = {}", b);
let c = 's';
println!("your var char = {}", c);
}
