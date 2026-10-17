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

fn main() {
	{
	let msg = Message::Write(String::from("Hello"));
	msg.call();

	println!("Square ");
	}
	
}

fn route() {

}


