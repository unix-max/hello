#[derive(Debug)]
struct Restangl {
	width: u32,
	height: u32,
}
fn main() {
	let width1 = 30;
	let height1 = 50;
	println!("Area {}", area(width1, height1));

	let rest1 =(30, 50);
	println!("Area1 {}", area1(rest1));

	let scale = 2;
	let rest2 = Restangl {
		width: dbg!(30 * scale),
		height: 50,
	};
	println!("Area2 {}", area2(&rest2));
	println!("Restangl {rest2:#?}");
	dbg!(&rest2);
}


fn area (width: i32, height: i32) -> i32 {
	width * height
	
}
fn area1 (dimensions: (i32, i32)) -> i32 {
	dimensions.0 * dimensions.1
	
}

fn area2 (rest: &Restangl) -> u32 {
	rest.width * rest.height
	
}
