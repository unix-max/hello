
fn main() {
	let roll = 9;
	match roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),
	}
	match roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => rello(),
	}
	match roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => (),
	}
}

fn add_fancy_hat() {}
fn remove_fancy_hat () {}
fn move_player(step:u8) {}
fn rello() {}

