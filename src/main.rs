enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChengeColor(i32,i32,i32),
}
struct QuitMessage;
struct MoveMessage {
	x:i32, y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(u32, u32, u32);
impl Message {
	fn call(&self) {

	}
}
enum Option<T> {
	Some(T),
	None,
}
fn main() {
	{
	let msg = Message::Write(String::from("Hello"));
	msg.call();
		let some_number = Option::Some(5);
		let some_char = Option::Some('s');
		let absent_number:Option<i32> = Option::None;
	println!("Square ");
	}
	let num = Option::Some(5);
	let num1 = 5;
	let z = num + num1;
	
}

fn route() {

}


