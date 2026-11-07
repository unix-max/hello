pub fn add_two(a: i32) -> i32 {
	println!("I got a value {a}");
	internal_adder(a, 2)
}

fn internal_adder(left: i32, right: i32) -> i32 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_two_test() {
		let res = add_two(2);
		assert_eq!(res, 4);
	}
	#[test]
	fn internal_adder_test() {
		let res = internal_adder(3, 2);
		assert_eq!(res, 5);
		
	}
	

	
}
	