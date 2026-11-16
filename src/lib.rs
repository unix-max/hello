//! # Create Hello
//! Учебный проет по написанию библиотеки поиска подстрок в тексте для реализации утилиты Grep
use std::env::{self};
pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}
impl Config {
	pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
		args.next();
		
	let query = match args.next() {
		Some(arg) => arg,
		None => return Err("Did not get query string")
	};
	let file_path = match args.next() {
		Some(arg) => arg,
		None => return Err("Didn`t get file name")
	};
	let ignore_case = env::var("IGNORE_CASE").is_ok();
	Ok(Config {query, file_path, ignore_case})
	} 
}
/// Осуществляет поиск подстроки в тексте с учетом регистра 
/// 
/// # Аргументы
/// * `query:`     &str - Указатель на искомой подстроки
/// *	`contents:`  &str - Указатель на текст в котором осуществляется поиск
/// 
/// # Возвращаемое значение
/// * `Vec<&str` - Вектор Указателей на строки содержащие искомую подстроку
/// 	
/// # Пример
/// 
/// ```
/// use hello::search_case_sensitive;
/// let query = "duct";
///	let contents = "\
///   Rust:
///   safe, fast, productive.
///   Pick three.
///   Duct tape.";
///	let result = search_case_sensitive(query, contents);
///	assert_eq!(vec!["  safe, fast, productive."], result);
/// ```
/// 
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

/// Осуществляет поиск подстроки в тексте без учета регистра 
/// 
/// # Аргументы
/// * `query:`     &str - Указатель на искомой подстроки
/// *	`contents:`  &str - Указатель на текст в котором осуществляется поиск
/// 
/// # Возвращаемое значение
/// * `Vec<&str` - Вектор Указателей на строки содержащие искомую подстроку
/// 	
/// # Пример
/// 
/// ```
/// use hello::search_case_insensitive;
/// let query = "rUsT";
///	let contents = "\
///   Rust:
///   safe, fast, productive.
///   Pick three.
///   Trust me.";
///	let result = search_case_insensitive(query, contents);
///	assert_eq!(vec!["Rust:", "  Trust me."], result);
/// ```
/// 
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
		.collect()
	
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		let result = search_case_sensitive(query, contents);
		assert_eq!(vec!["safe, fast, productive."], result);
	}

#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		let result = search_case_insensitive(query, contents);
		assert_eq!(vec!["Rust:", "Trust me."], result);
	}

	
}
	