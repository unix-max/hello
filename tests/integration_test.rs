use hello::add_two;
mod common;

#[test]
fn it_adds_two() {
	common::setup();
	let res = add_two(2);
	assert_eq!(res, 4);
}