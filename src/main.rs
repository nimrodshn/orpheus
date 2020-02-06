use std::io;
use std::env;
use std::process;
use crate::memtable::Memtable;
use crate::config::Config;
use std::error::Error;

extern crate clap;

pub mod memtable;
pub mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let mut memtable = Memtable::from_config(conf).unwrap_or_else(|err| {
        println!("Failed to create a memtable: {}", err);
        process::exit(1);
    });

    Ok(())
}
