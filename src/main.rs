
struct CustomSmartPointer {
	data: String
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Droping CustomSmartPointer whith data {}", self.data);
	}
}
fn main() {
	let c = CustomSmartPointer {
		data: String::from("My stuff")
	};
	println!("CustomSmartPointer C created !");
	drop(c);
	let d = CustomSmartPointer {
		data: String::from("Other stuff")
	};
	println!("CustomSmartPointer created !");
}

