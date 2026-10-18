#[derive(Debug)]
enum usState {
		Alabama,
  	Alaska,
}
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(usState),	
}
fn main() {
	let coin = Coin::Quarter(usState::Alaska);
	let price = coin_to_cents(coin);
	println!("Coin price {}",  price);
	
}

fn coin_to_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky penny");
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State {state:?}");
			25
		},
	}

}


