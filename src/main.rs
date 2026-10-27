struct Point<T> {
	x: T,
	y: T,
}
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
	fn y(&self) -> &T {
		&self.y
	}
}
impl Point<f32> {
	fn distance(&self) -> f32 {
		(self.x.powi(2)+ self.y.powi(2)).sqrt()
	}
}
struct  Point1<T,U>{
	x: T,
	y: U,
}
struct Point3<X1, Y1> {
	x: X1,
	y: Y1,
}
impl<X1, Y1> Point3<X1, Y1> {
	fn cross<X2, Y2>(self, other:Point3<X2,Y2>) -> Point3<X1, Y2>{
		Point3 { x: self.x, y: other.y }
	}
}

enum options<T> {
	Some(T),
	None,
}
enum Result1<T,E> {
	Ok(T),
	Err(E),
}
fn main() {
	let ineger = Point{x:1, y:2};
	println!("x={}, y={}", &ineger.x(), &ineger.y());
	let float = Point{x: 3.0, y: 4.0};
	let dist = float.distance();
	println!("dist float ={}", dist);
	let combo = Point1{x:1, y:4.0};
	let p1 = Point3{x: 3, y: 4.9};
	let p2 = Point3{x: 3.2, y: 5};
	let p3 = p1.cross(p2);
println!("P3 x= {}, y={}", p3.x, p3.y);
}

