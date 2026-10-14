
struct User {
	activ: bool,
	user_name: String,
	email: String,
	sign_in_count: u64,
}
struct StrUser {
	activ: bool,
	user_name: &str,
	email: &str,
	sign_in_count: u64,
}
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

struct Alwaysiqual;
fn main() {
	let mut user1 = build_user(
		String::from("Max Mdoy"),
		String::from("unix_max@mail.ru"));
	user1.email = String::from("max@mdoy.pro");

	let user2 = User {
		activ: user1.activ,
		user_name: String::from("Harry Harison"),
		email: user1.email,
		sign_in_count: 2,
	};
	let user3 = User {
		email: String::from("Sveta Acrop"),
		sign_in_count: 3,
		..user1
	};
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	let subject = Alwaysiqual;

	let user4 = StrUser {
		activ: true,
		user_name: "Hello world",
		email: "max@mdoy",
		sign_in_count: 5,
	};
}


fn build_user (user_name: String, email: String) -> User {
	User {
		activ: true,
		user_name,
		email,
		sign_in_count: 1,
	}
}

