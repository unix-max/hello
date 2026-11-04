#[derive(Debug)]
struct Restangl {
	width: u32,
	height: u32,
}
impl Restangl {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn width (&self) -> bool {
		self.width >0
	}
	fn can_hold (&self, other: &Restangl) -> bool {
		self.width < other.width && self.height > other.height
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ladger_can_hold_smolar() {
		let larger = Restangl {
			width: 7,
			height: 8
		};
		let smaller = Restangl {
			width: 5,
			height: 1
		};
		assert!(larger.can_hold(&smaller));
	}

	#[test]
	fn smaller_cannot_hold_ladger() {
		let larger = Restangl {
			width: 7,
			height: 8
		};
		let smaller = Restangl {
			width: 5,
			height: 1
		};
		assert!(!smaller.can_hold(&larger));
	}

}
	