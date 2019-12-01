use std::collections::HashMap;
use std::fs::File;
use std::io;
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
    pub fn new(path : &Path) -> Result<Memtable, io::Error> {
        let file = OpenOptions::new().
            write(true).
            append(true).
            create(true).
            open(path)?;

        let result = Memtable{
            index: HashMap::new(),
            log: file,
            offset: 0,
        };
        Ok(result)
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), io::Error> {
        let value_raw = value.as_bytes();
        self.log.write(value_raw)?;
        
        let mem_table_entry = Entry{offset:self.offset, size: value_raw.len()};
        self.index.insert(key, mem_table_entry);
        self.offset = self.offset + value_raw.len();
        Ok(())
    }
}