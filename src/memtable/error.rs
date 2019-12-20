use std::fmt;
use std::io;
use crate::memtable::Memtable;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub enum Error {
    Io(std::rc::Rc<io::Error>),
    FromUTF8(std::rc::Rc<std::string::FromUtf8Error>),
    NotFound,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Some error occurred!")
    }
}

// a test function that returns our error result
fn raises_my_error(mut memtable: Memtable, key: String) -> Result<(),Error> {
    match memtable.read(&key) {
        Ok(_v) => Ok(()),
        Err(e) => Err(e),
    }
}
