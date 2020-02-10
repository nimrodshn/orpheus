use std::process;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

extern crate clap;
use clap::{Arg, App};

use crate::memtable::Memtable;
use crate::config::Config;

pub mod memtable;
pub mod config;

fn main() {
    let matches =App::new("orpheus")
        .version("1.0")
        .author("Nimrod Shneor <nshneor@redhat.com>")
        .about("Experimental key value storage engine")
        .arg(
            Arg::with_name("log_path")
            .long("log_path")
            .value_name("PATH")
            .help("Sets a path for the log file")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("port")
            .long("port")
            .value_name("PORT")
            .help("The port file to listen to requests")
            .takes_value(true)
            .required(true)
        ).get_matches();
    
    
    let path = matches.value_of("log_path").unwrap().to_string();
    let port = matches.value_of("port").unwrap().to_string();

    let conf = Config::new(port, path);
    
    if let Err(e) = run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let host = format!("127.0.0.1:{}", conf.port);
    let listener = TcpListener::bind(host).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)?;
    }

    let mut memtable = Memtable::from_config(conf).unwrap_or_else(|err| {
        println!("Failed to create a memtable: {}", err);
        process::exit(1);
    });

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(),String>{
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Err(e) => return Err(format!("Error occured while attempting to read request: {}",e)),
        Ok(v) => v
    };

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    match stream.write(response.as_bytes()) {
        Err(e) => return Err(format!("Could not write a response, {}", e)),
        Ok(v) => v
    };
    stream.flush().unwrap();
    Ok(())
}
