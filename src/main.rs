use std::thread;
fn main() {

	let list = vec![1, 2, 3, 7];
	println!(" Before defining closure {list:?}");

	thread::spawn(move || println!("From closure {list:?}"))
		.join()
		.unwrap();
	
	//println!("After calling closure {list:?}");
}




