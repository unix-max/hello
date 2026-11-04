fn add(left: u32, right: u32) -> u32 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_work() {
		let res = it_works();
		let is_err = res.is_err();
		assert!(!is_err);
	}

	fn it_works() -> Result<u32, String> {
		let result = add(2, 2);
		if result ==4 {
			Ok(result)
		} else {
			Err(String::from("Must be 4 bur return {result}"))
		}
		
	}

	
}
	