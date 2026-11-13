pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
fn main() {
let v1 = vec![1,2,3];
let mut v1_iter = v1.iter();
let a = v1_iter.next().unwrap_or_else(|| {println!("end iter"); &0});
println!("Value iter 1 {a}");
let a = v1_iter.next().unwrap_or_else(|| {println!("end iter"); &0});
println!("Value iter 2 {a}");
let a = v1_iter.next().unwrap_or_else(|| {println!("end iter"); &0});
println!("Value iter 3 {a}");
let a = v1_iter.next().unwrap_or_else(|| {println!("end iter"); &0});
println!("Value iter 4 {a}");


}




