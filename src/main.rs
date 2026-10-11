
fn main() {
	let arr = [10, 20, 30, 40, 50];
	let mut count =0;
	while count < 5 {
		println!("arr elm {}", arr[count]);
		count += 1;
	}
		
		println!("final count {count}");

		for elm in arr {
			println!("arr elm {elm}");
		}
		for number in (1..10).rev() {
			println!("number {number}");
		}
	}
	