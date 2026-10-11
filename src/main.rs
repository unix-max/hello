
fn main() {
	let number = 7;
	if number < 5 {
		println!("condition was true");
	} else {
		println!("conditions was false");
	} 
	if number != 0 {
		println!("Nomber was`n 0");
	}

	if number % 4 == 0 {
		println!("Число делится на 4");
	} else if number % 3 ==0 {
		println!("Число делится на 4");
	} else if number % 2 ==0 {
		println!("Число делится на 2");
	} else {
		println!("Число не делится на 4, 3, 2");
	}

	let condition = true;
	let number = if condition {5} else { 6};
	println!(" Number {number}");
	
}
