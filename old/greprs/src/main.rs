extern crate greprs;

use std::env;
use std::process;

use greprs::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In File {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error : {}", e);
        process::exit(1);
    }
}