
#[derive(Debug)]
pub struct Config {
    pub log_path: String,
    pub port: String,
}

impl Config {
    pub fn new(port: String, log_path: String) -> Config {
        Config{
            log_path: log_path,
            port: port,
        }
    }
}