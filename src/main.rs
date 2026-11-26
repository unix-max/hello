use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
	value: i32,
	parrent: RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>
}


fn main() {
	let leaf = Rc::new(Node {
		value: 10,
		parrent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![])
	});
	println!("leaf parrent = {:?}", leaf.parrent.borrow().upgrade());
	println!("Liaf strong = {}, weak = {}\n", Rc::strong_count(&leaf), Rc::weak_count(&leaf));	
	{
		let branch = Rc::new(Node {
		value: 25,
		parrent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![Rc::clone(&leaf)])
	});
	*leaf.parrent.borrow_mut() = Rc::downgrade(&branch);

	println!("leaf parrent = {:?}", leaf.parrent.borrow().upgrade());
	println!("Liaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));	
	println!("Branch strong = {}, weak = {}\n", Rc::strong_count(&branch), Rc::weak_count(&branch));
	}
	println!("leaf parrent = {:?}", leaf.parrent.borrow().upgrade());
	println!("Liaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));	
		
}