extern crate greps;

use std::env;
use std::process;

use greps::Config;
use greps::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    if config.is_err() {
        println!("Problem parsing arguments: {}", config.err().unwrap());
        process::exit(1);
    }

    let c = config.unwrap();

    println!("Searching for {}", c.query);
    println!("In file {}", c.filename);

    if let Err(e) = run(c) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}