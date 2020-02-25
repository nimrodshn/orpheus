use std::process;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::io;
use std::net::TcpStream;
use std::path::Path;

extern crate clap;
use clap::{Arg, App};

use orpheus::config::Config;
use orpheus::thread_pool::ThreadPool;
use orpheus::server::APIServer;

#[tokio::main]
async fn main() {
    let matches =App::new("orpheus")
        .version("1.0")
        .author("Nimrod Shneor <nshneor@redhat.com>")
        .about("Experimental key value storage engine")
        .arg(
            Arg::with_name("log-path")
            .long("log-path")
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
    
    
    let path = matches.value_of("log-path").unwrap().to_string();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    let conf = Config::new(
        port,
        path,
    );
    
    if let Err(e) = run_server(conf).await {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

async fn run_server(conf: Config) -> Result<(), io::Error> {
    let api_server = APIServer::new(conf.port, conf.log_path)?;
    api_server.run().await;
    Ok(())
}