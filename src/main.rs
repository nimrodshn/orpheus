use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // Define the log file to append to
    let path = Path::new("/home/nimrodshneor/log.txt");
    let display = path.display();
    let offset : usize = 0;
    let mut mem_table = HashMap::new(); 

    println!("Please enter the next key");
    let mut key = String::new();
    io::stdin().read_line(&mut key)
        .expect("Failed to read line");

    println!("Please enter the next values");
    let mut value = String::new();
    io::stdin().read_line(&mut value)
        .expect("Failed to read line");
    
    let value_raw = value.as_bytes();
    
    // Write key value pair to file.
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(value_raw) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("succesfuly written to {}", display)
    };

    mem_table.insert(key, offset);

    // increment offset by raw.len():
    // offset = offset + value_raw.len();
}