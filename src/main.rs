#[derive(Debug)]
enum UsState {
		Alabama,
  	Alaska,
}
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),	
}
fn main() {
	let mut count = 0;
	let coin = Coin::Quarter(UsState::Alabama);
	match &coin {
		Coin::Quarter(state) => {
			println!("coin state {state:?}");
		},
		_ => count +=1,
	}
	println!("Count {count}");
	//let coin = Coin::Quarter(UsState::Alabama);
	if let Coin::Quarter(state) = coin {
		println!("coin state {state:?}");
	} else {
		count +=1;
	}
	println!("Count {count}");
}


