use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil,
}
use crate::List::{Cons, Nil};
fn main() {
	let value = Rc::new(RefCell::new(5));
	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
	
	let b = Cons(Rc::new(RefCell::new(11)), Rc::clone(&a));
	
	{
		let c = Cons(Rc::new(RefCell::new(12)), Rc::clone(&a));
		println!("RC count after creating a= {}", Rc::strong_count(&a));
		*value.borrow_mut() += 10;
		println!("a after = {a:?}");
    println!("b after = {b:?}");
		if let Cons(x, _) = &b {
			*x.borrow_mut() +=12;
		};
		println!("b after after = {b:?}");
    println!("c after = {c:?}");
	}
	println!("a after = {a:?}");
    println!("b after = {b:?}");
    
}