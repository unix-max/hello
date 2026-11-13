#[derive(Debug)]
struct Restangle {
	width: u32,
	height: u32
}
fn main() {
	let mut list = [
		Restangle { width: 10, height: 1},
		Restangle { width: 3, height: 5},
		Restangle { width: 7, height: 12}
	];
	let mut sort_operation = vec![];
	let value = String::from("Closure called");

	list.sort_by_key(|r| {
		sort_operation.push(value);
		r.width
	});
	println!("list {list:#?}")
}




