
#[derive(Debug)]
pub struct Config {
    pub log_path: String,
    pub port: String,
    pub thread_pool_size: usize,
}

impl Config {
    pub fn new(port: String, log_path: String, thread_pool_size: usize) -> Config {
        Config{
            log_path,
            port,
            thread_pool_size,
        }
    }
}