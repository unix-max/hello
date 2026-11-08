use std::env;
use std::error::Error;
use std::fs;
use std::process;
use hello::Config;
use hello::{search_case_sensitive, search_case_insensitive};
fn main() {
	let args:Vec<String> = env::args().collect();
	let config = Config::build(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {err}");
		process::exit(1);
	});
	println!("Serching for {}", config.query);
	println!("In file {}", config.file_path);
	if let Err(e) = run(config) {
		eprintln!("Application error {e}");
		process::exit(1);
	}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;
	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search_case_sensitive(&config.query, &contents)
	};
	for line in results {
		println!("whith text {line}");
	}
	
	Ok(())
}



