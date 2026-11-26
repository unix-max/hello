use std::thread;
use std::time::Duration;
fn main() {
	let v = vec![1, 2, 3, 4, 5];
	let handle = thread::spawn(move || {
		for (i, item) in v.iter().enumerate() {
			println!("Member number {i} is {item}");
			thread::sleep(Duration::from_millis(1));
		}
	});
	
	handle.join().unwrap();
	
}