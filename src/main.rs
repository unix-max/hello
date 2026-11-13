
fn main() {

	let add_two = |x: i32| -> i32 { x + 2};
	let z = add_two(2);
	println!(" this is {z}");
	let example_closure = |x| x;
	let s1 = example_closure(String::from("Hello"));
	println!("1 is {s1}");
	//let n1 = example_closure(5);
	let mut list = vec![1,2,3];
	println!(" Before defining closure {list:?}");
	let mut only_borrows = || {
		list.push(7);
		println!("From closure {list:?}");
	};

	//println!("Before calling closure {list:?}");
	only_borrows();
	println!("After calling closure {list:?}");
}




