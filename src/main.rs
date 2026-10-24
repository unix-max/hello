use std::collections::HashMap;
fn main() {
	
	{
		let mut numbers = vec![5, 2, 8, 1, 3, 12, 17, 11, 7];
		numbers.sort();
	//	let text = "hello world wonderful world";
		let mut map = HashMap::new();
		let mut summ: i32 = 0;
		let len = numbers.len();
		let mut len1: i32 = 0;
		match len.try_into() {
			 Ok(val) => len1 = val,
			 Err(_) => println!("Ошибка: значение usize слишком велико для i32"),				
		}

		for i in &numbers {
			let count = map.entry(i).or_insert(0);
			*count +=1;
			summ +=i;
		}
		println!("{numbers:?}");
		println!("{map:?}");
		println!("Minimal {}",numbers.first().unwrap());
		println!("Maximal {}", numbers.last().unwrap());
		println!("srednee {}", numbers.last().unwrap()/2);
		if len > 0 {
			println!("median {}",numbers[len/2]);
			println!("sred afriav {}", summ/len1);
		} 
	}
	
	println!("Count End");
	
}
