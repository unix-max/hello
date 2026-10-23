use std::collections::btree_map::Values;

enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		String(String),
	}
fn main() {
	let mut v: Vec<i32> = Vec::new();
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	let third = &v[2];
	
	println!("Third val {}", third);
	v.push(10);

	let third: Option<&i32> = v.get(2);
	match third {
		Some(third) => println!("third value {third}"),
		None => println!("this is no third element"),
	}
	
	let third = &v[3];
	println!("3 val {}", third);

	let third: Option<&i32> = v.get(100);
	match third {
		Some(third) => println!("third value {third}"),
		None => println!("this is no third element"),
	}
	for i in &mut v {
		*i += 50;
		println!("{i}");
	}
	for i in &v {
		println!("{i}");
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(6.15),
		SpreadsheetCell::String(String::from("blue")),
	];
	for i in &row {
		match i {
			SpreadsheetCell::Int(e) => {
				println!("int {e}");
			},
			SpreadsheetCell::Float(e) => {
				println!("Float {e}");
			},
			SpreadsheetCell::String(e) => {
				println!("String {e}");
			}
		}
	}
	println!("Count");
	
}
