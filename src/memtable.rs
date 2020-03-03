use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::rc::Rc;
use crate::memtable::error::Error;
pub mod error;

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
            read(true).
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

    pub fn write(&mut self, key: String, value: String) -> Result<(), io::Error> {
        let value_raw = value.as_bytes();
        self.log.write(value_raw)?;
        
        let mem_table_entry = Entry{offset:self.offset, size: value_raw.len()};
        self.index.insert(key, mem_table_entry);
        self.offset = self.offset + value_raw.len();
        Ok(())
    }

    pub fn read(&mut self, key: &String) -> Result<String, Error> {
        let entry = match self.index.get(key) {
            None => return Err(Error::NotFound),
            Some(v) => v,
        };
        let mut buf = vec![0u8; entry.size];
        // Seek the offset of the value in the log file.
        match self.log.seek(io::SeekFrom::Start(entry.offset as u64)) {
            Err(e) => return Err(Error::Io(Rc::new(e))),
            _ => (),
        };
        // Read the the lines from file to a byte buffer. 
        match self.log.read_exact(&mut buf) {
            Err(e) => return Err(Error::Io(Rc::new(e))),
            Ok(v) => v,
        };
        // Convert the byte buffer to string and return to the user.
        let result = match String::from_utf8(buf) {
            Err(e) => return Err(Error::FromUTF8(Rc::new(e))),
            Ok(v) => v,
        };
        Ok(result)
    }
}