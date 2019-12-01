use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

#[derive(Debug)]
struct Entry {
    offset: usize,
    size: usize,
}

#[derive(Debug)]
pub struct Memtable {
    offset: usize,
    index: HashMap<String, Entry>,
    log: File,
}

impl Memtable {
    pub fn new(path : &Path) -> Result<Memtable, String> {
        let file = match OpenOptions::new().
            write(true).
            append(true).
            create(true).
            open(path) {
                Err(why) => return Err(format!("Could not open a file: {}", why)),
                Ok(file) => file,
            };

        let result = Memtable{
            index: HashMap::new(),
            log: file,
            offset: 0,
        };
        Ok(result)
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), &'static str> {
        let value_raw = value.as_bytes();
        match self.log.write(value_raw) {
            Err(_) => return Err("couldn't write to file"),
            _ => (),
        };

        let mem_table_entry = Entry{offset:self.offset, size: value_raw.len()};
        self.index.insert(key, mem_table_entry);
        self.offset = self.offset + value_raw.len();
        Ok(())
    }
}