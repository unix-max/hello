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
		self.width > other.width && self.height > other.height
	}
}
impl Restangl {
	fn square(size: u32) -> Self {
		Self { width: size, height: size }
	}
}
fn main() {
	let rest1 = Restangl {
		width: 30,
		height: 50,
	};
	println!("Area {}", rest1.area());
	println!("Width >0 {}", rest1.width());
	let rest2 = Restangl {
		width: 10,
		height: 40,
	};
	let rest3 = Restangl {
		width: 60,
		height: 45,
	};
	println!(" Restangl 2 can hold in rest1 {}", rest1.can_hold(&rest2));
	println!(" Restangl 3 can hold in rest1 {}", rest1.can_hold(&rest3));
	let square1 = Restangl::square(10);
	println!("Square {square1:#?}");
}


