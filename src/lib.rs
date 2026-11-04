pub struct Guess{
	value: i32
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value<1 {
			panic!("value mast be > 1, got{value}");
		} else if  value > 100 {
			panic!("value mast be < 100, got{value}");
		}
		Guess { value }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "value mast be < 100")]
	fn greater_then_100() {
		Guess::new(200);
		
	}

	
}
	