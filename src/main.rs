use std::thread;
use std::time::Duration;
fn main() {
	let handle = thread::spawn(|| {
		for i in 0..20 {
			println!("hi number {i} from the spawn thread");
			thread::sleep(Duration::from_millis(1));
		}
	});
	for i in 0..5 {
			println!("hi number {i} from the main thread");
			thread::sleep(Duration::from_millis(5));
			
		}
	handle.join().unwrap();
}