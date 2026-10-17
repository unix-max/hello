enum IpAddr {
		V4(u8, u8, u8, u8),
		V6(String),
	}
struct Ipv4Addr{
	addr: String,
}
struct Ipv6Addr {
	addr: String,
}
enum IpAddr1 {
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}
fn main() {
	{
		let home = IpAddr::V4(127,0,0,1);
		let loopback= IpAddr::V6(String::from("::1"));

	println!("Square ");
	}
	
}

fn route(ip_addr: IpAddr) {

}


