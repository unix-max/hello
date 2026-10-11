use std::io;
fn main() {
 
 let tup:(u16, u32, f32) = (32, 12, 3.12);
 let (a, b, c,) = tup;
 println!("Кортеж tup {}, {}, {}", a, b, c);
 let a = tup.0;
 let b = tup.1;
 let c = tup.2;
 println!("Кортеж tup {}, {}, {}", a, b, c);

 let x=[1,2,3,4,5];
 for (index, elm) in x.iter().enumerate() {
	println!("Элемент номер {index} {elm}");
 }
 let months = ["January", "February", "March", "April", "May", "June",
 	"July", "August", "September", "October", "November", "Disember"];
for (index, month) in months.iter().enumerate() {
	println!("Элемент номер {index} {month}");
 }
 let d=[3;5];
 for (index, var) in d.iter().enumerate() {
	println!("Элемент номер {index} {var}");
 }
 let i = 1;
 let first = d[i];
 println!("This is first {}", first);

 println!("Inter month number");
 let mut index = String::new();
 io::stdin()
 .read_line(&mut index)
 .expect("Faild to read line");

 let index:usize = index
 	.trim()
 	.parse()
 	.expect("This is not a number");
let len = months.len();
if index <= len {
	let elm = months[index];
	println!("Your month is {}", elm);
} else {
	println!("Индекс больше длины массива {}", len);
}

}
