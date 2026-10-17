enum IpAddrKind {
		V4,
		V6
	}

struct IpAddr {
	kind: IpAddrKind,
	addr: String,
}
fn main() {
	{
	let v4 = IpAddrKind::V4;
	let v6= IpAddrKind::V6;

	route(v4);
	route(v6);

	println!("Square ");
	}
	{
		let home = IpAddr {
			kind: IpAddrKind::V4,
			addr: String::from("192.168.0.10"),
		};
		let loopback = IpAddr {
			kind: IpAddrKind::V6,
			addr: String::from("::1"),
		};
	}
}

fn route(ip_kind: IpAddrKind) {

}


