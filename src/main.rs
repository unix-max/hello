use std::rc::Rc;

enum List {
	Cons(i32, Rc<List>),
	Nil,
}
use crate::List::{Cons, Nil};
fn main() {
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	println!("RC count after creating a= {}", Rc::strong_count(&a));
	let b = Cons(3, Rc::clone(&a));
	println!("RC count after creating a= {}", Rc::strong_count(&a));
	{
		let c = Cons(4, Rc::clone(&a));
		println!("RC count after creating a= {}", Rc::strong_count(&a));
	}
	println!("RC count after creating a= {}", Rc::strong_count(&a));
}