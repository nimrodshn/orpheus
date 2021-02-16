use crate::memtable::error::Error;
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
pub mod error;

#[derive(Debug)]
struct Entry {
    offset: usize,
    size: usize,
}

#[derive(Debug)]
pub struct Memtable {
    offset: usize,
    index: BTreeMap<String, Entry>,
    log: File,
}

impl Memtable {
    pub fn new(path: &Path) -> Result<Memtable, io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;

        let result = Memtable {
            index: BTreeMap::new(),
            log: file,
            offset: 0,
        };
        Ok(result)
    }

    pub fn write(&mut self, key: String, value: String) -> Result<(), io::Error> {
        let value_raw = value.as_bytes();
        self.log.write(value_raw)?;

        let mem_table_entry = Entry {
            offset: self.offset,
            size: value_raw.len(),
        };
        self.index.insert(key, mem_table_entry);
        self.offset = self.offset + value_raw.len();
        Ok(())
    }

    pub fn read(&self, key: &String) -> Result<String, Error> {
        let entry = match self.index.get(key) {
            None => return Err(Error::NotFound),
            Some(v) => v,
        };
        let mut buf = vec![0u8; entry.size];

        // TODO Consider guarding the log file with Arc.
        let mut cloned_file = match self.log.try_clone() {
            Err(e) => return Err(Error::Io(Rc::new(e))),
            Ok(v) => v,
        };

        // Seek the offset of the value in the log file.
        match cloned_file.seek(io::SeekFrom::Start(entry.offset as u64)) {
            Err(e) => return Err(Error::Io(Rc::new(e))),
            _ => (),
        };
        // Read the the lines from file to a byte buffer.
        match cloned_file.read_exact(&mut buf) {
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_read() -> Result<(), String> {
        use crate::memtable::{Entry, Memtable};
        use std::io::Write;
        use std::path::Path;
        let mut memtable = match Memtable::new(Path::new("/tmp/log")) {
            Ok(memtable) => memtable,
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to create a log file: {}",
                    e
                )))
            }
        };

        let key = String::from("hello");
        let value = String::from("world");
        let offset = 0;
        match memtable.log.write(value.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to write to log file: {}",
                    e
                )))
            }
        };
        memtable.index.insert(
            key.clone(),
            Entry {
                offset: offset,
                size: value.len(),
            },
        );

        let res = match memtable.read(&key) {
            Ok(val) => val,
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to read from memtable: {}",
                    e
                )))
            }
        };
        assert_eq!(res, value);
        Ok(())
    }

    #[test]
    fn test_write() -> Result<(), String> {
        use crate::memtable::Memtable;
        use std::io::{Read, Seek, SeekFrom};
        use std::path::Path;
        let mut memtable = match Memtable::new(Path::new("/tmp/log")) {
            Ok(memtable) => memtable,
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to create a log file: {}",
                    e
                )))
            }
        };

        let key = String::from("hello");
        let value = "world";
        match memtable.write(key.clone(), String::from(value)) {
            Ok(_) => (),
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to write to log file: {}",
                    e
                )))
            }
        };

        let mut bytes = vec![0x00;value.len()];
        let entry = match memtable.index.get(&key) {
            Some(entry) => entry,
            None => {
                return Err(String::from(format!(
                    "Could not fine entry in index for key: {}",
                    key
                )))
            }
        };

        if let Err(e) = memtable
            .log
            .seek(SeekFrom::Start(entry.offset as u64))
            .and_then(|_| memtable.log.read_exact(&mut bytes))
        {
            return Err(String::from(format!(
                "Faild to read value from file: {}",
                e
            )));
        };

        let res = match String::from_utf8(bytes) {
            Ok(val) => val,
            Err(e) => {
                return Err(String::from(format!(
                    "An error occurred while trying to read from memtable: {}",
                    e
                )))
            }
        };

        assert_eq!(res, value);
        Ok(())
    }
}
