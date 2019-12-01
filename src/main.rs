use std::io;
use std::path::Path;
use orpheus::Memtable;

fn main() {
    let path = Path::new("/home/nimrodshneor/log.txt");

    loop {
        println!("Please enter the next key");
        let mut key = String::new();
        io::stdin().read_line(&mut key)
            .expect("Failed to read line");

        println!("Please enter the next values");
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        
        let mut mem_table = Memtable::new(path).expect("Failed to create a mem table");
        
        mem_table.insert(key, value).expect("Could not write to mem table");
    }
}