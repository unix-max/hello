fn greeting(name: &str) -> String {
	format!("Hello !")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn greeting_control_name() {
		let result = greeting("Carol");
		assert!(result.contains("Carol")," walue was {result}");
	}

	
}
	