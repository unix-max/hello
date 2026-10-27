
fn main() {
	let num_list = [1,2,3,4,5,6,7,8];
	let result = biggest_i32(&num_list);
	println!("Bigest int32 {result}");

	let char_list = ['a', 'b', 'c', 'e', 's'];
	let result = biggest_char(&char_list);
	println!("Bigest char {result}");

	let result = bigest(&num_list);
	println!("Bigest int32 {result}");
	let result = bigest(&char_list);
	println!("Bigest char {result}");
}

fn biggest_i32(list: &[i32]) -> &i32 {
    let mut bigest = &list[0];
    for num in list {
      if num > &bigest {
				bigest = num;
			}
    }
		bigest
}

fn biggest_char(list: &[char]) -> &char {
    let mut bigest = &list[0];
    for num in list {
      if num > &bigest {
				bigest = num;
			}
    }
		bigest
}

fn bigest<T: PartialOrd>(list: &[T]) -> &T {
	let mut bigest = &list[0];
    for num in list {
      if num > &bigest {
				bigest = num;
			}
    }
		bigest
}