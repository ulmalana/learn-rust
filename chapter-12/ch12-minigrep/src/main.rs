use std::env;
use std::process;

use ch12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    //println!("### Result ###");

    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    //println!("{:?}", args);
}
