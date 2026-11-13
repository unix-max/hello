#[derive(Debug)]
struct Restangle {
	width: u32,
	height: u32
}
fn main() {
	let mut list = [
		Restangle { width: 10, height: 1},
		Restangle { width: 3, height: 5},
		Restangle { width: 7, height: 12},
		Restangle { width: 9, height: 12},
		Restangle { width: 10, height: 1},
		Restangle { width: 3, height: 5},
		Restangle { width: 7, height: 12},
		Restangle { width: 9, height: 12}
	];
	
	let mut nom = 0;

	list.sort_by_key(|r| {
		nom +=1;
		r.width
	});
	println!("list {list:#?} sorted in {nom} sort operation");
}




