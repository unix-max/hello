fn add_two(a: u32) -> u32 {
	println!("I got a value {a}");
	a + 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_two_and_two() {
		let res = add_two(2);
		assert_eq!(res, 4);
	}
	#[test]
	fn add_two_and_three() {
		let res = add_two(3);
		assert_eq!(res, 5);
		
	}
	#[test]
	fn one_hundred() {
		let res = add_two(100);
		assert_eq!(res, 102);
		
	}
	#[test]
	#[ignore = "reason"]
	fn expensive_test() {
		let res = add_two(100);
		assert_eq!(res, 102);
	}

	
}
	