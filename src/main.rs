use std::io;
use std::path::Path;
use crate::memtable::Memtable;

pub mod memtable;

fn main() {
    let path = Path::new("/home/nshneor/workspace/log.txt");
    let mut mem_table = Memtable::new(path).expect("Failed to create a mem table");

    loop {
        println!("Please enter the next key");
        let mut key = String::new();
        io::stdin().read_line(&mut key)
            .expect("Failed to read line");

        println!("Please enter the next values");
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        
        mem_table.write(key, value).expect("Could not write to mem table");
        
        println!("Please enter the next key to read from");
        let mut key = String::new();
        io::stdin().read_line(&mut key)
            .expect("Failed to read line");
            
        let value = match mem_table.read(&key) {
            Ok(v) => v,
            Err(e) => panic!("An error occured, while trying to read message: {:?}", e),
        };

        println!("The value corresponding to key {} is {}", key, value);

    }
}