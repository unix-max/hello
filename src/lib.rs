fn print_and_return_10(a: u32) -> u32 {
	println!("I got a value {a}");
	10
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_will_pass() {
		let res = print_and_return_10(4);
		assert_eq!(res, 10);
	}
	#[test]
	fn test_will_fail() {
		let res = print_and_return_10(8);
		assert_eq!(res, 5);
		
	}

	
}
	