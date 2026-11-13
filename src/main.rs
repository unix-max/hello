#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
	Red,
	Blue,
}
struct Inventory {
	shirts: Vec<ShirtColor>
}

impl Inventory {
	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked())
	}

	fn most_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;
		for color in &self.shirts {
			match color {
				ShirtColor::Red => num_red += 1,
				ShirtColor::Blue => num_blue += 1
			}
		}
		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}
}
fn main() {
	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};
	let user_profile1 = Some(ShirtColor::Red);
	let give_away = store.giveaway(user_profile1);
	println!("User with preference {:?} gives {:?}", user_profile1, give_away);
	let user_profile2 = None;
	let give_away = store.giveaway(user_profile2);
	println!("User with preference {:?} gives {:?}", user_profile2, give_away);
	let add_two = |x: i32| -> i32 { x + 2};
	let z = add_two(2);
	println!(" this is {z}");
}




