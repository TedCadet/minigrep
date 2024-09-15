use std::env;
use std::process::exit;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        exit(1);
    }
}
