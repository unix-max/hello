pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
fn main() {
let v1 = vec![1,2,3];
let v1_map  = v1.iter().map(|x| x+1);
let sum: i32 = v1_map.sum();
println!("Summ of iter {sum}");
let v1_map  = v1.iter().map(|x| x+1);
let v2: Vec<i32> = v1_map.collect();
println!("New collect from map {v2:?}")

}




