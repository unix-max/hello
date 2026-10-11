
fn main() {
	let mut count = 0;
	'counting: loop {
		println!("count {count}");
		let mut remind = 10;
			loop {
				println!("remind {remind}");
				if remind == 1 {
					break;
				}
				if count == 3 {
					break 'counting;
				}
				remind -=1;
			};
			count +=1;
			
		}
		count += 1;
		println!("final count {count}");
	}
	