use std::env;
use std::error::Error;
use std::fs;
use std::process;
use hello::Config;
use hello::search;
fn main() {
	let args:Vec<String> = env::args().collect();
	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		process::exit(1);
	});
	println!("Serching for {}", config.query);
	println!("In file {}", config.file_path);
	if let Err(e) = run(config) {
		println!("Application error {e}");
		process::exit(1);
	}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;
	for line in search(&config.query, &contents) {
		println!("whith text {line}");
	}
	
	Ok(())
}



