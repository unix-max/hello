//use std::collections::HashMap;
fn main() {

	let source_text = "Tokio is an asynchronous runtime for the Rust programming language.
		It provides the building blocks needed for writing networking applications.
		It gives the flexibility to target a wide range of systems,
		from large servers with dozens of cores to small embedded devices.";
	let mut result_text = String::new();

	let words: Vec<&str> = source_text.split_whitespace().collect();
	for word in words {
		let last_char = word.chars().last().unwrap();
		let wd: &str = if is_punctuation(&last_char) {
			let end_index = word.char_indices().next_back().map_or(0, |(i, _)| i);
			&word[..end_index]
		
		} else { word };
		let mut new_word = String::new();
		for (index, c) in wd.chars().enumerate() {
			if is_vowel(c) {
				let rest_wd:String = wd.chars().skip(index).collect();
				let a = format!("{}{}{}", rest_wd, new_word, "yu");
				result_text.push_str(&a); 
				break;
			} else {
				new_word.push(c);
			}
		}
		if is_punctuation(&last_char) { 
			result_text.push(last_char);
		}
		result_text.push_str(" ");
	}
	println!("{}", result_text);
	println!("Count End");	
}

fn is_vowel(c: char) -> bool {
    "aeiouAEIOU".contains(c)
}
fn is_punctuation(c: &char) -> bool {
	".,!?:;-".contains(*c)
}
