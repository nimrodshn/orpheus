#[derive(Debug)]
pub struct Config {
    pub log_path: String,
    pub port: u16,
}

impl Config {
    pub fn new(port: u16, log_path: String) -> Config {
        Config { log_path, port }
    }
}
