
#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub log_path: String,
    pub segment_size: u8,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 4 {
            return Err(String::from("not enough arguments!"));
        }
        let segment_size = match args[3].clone().parse::<u8>() {
            Err(err) => {
                let err_str = String::from(format!("An error occurred trying to parse segment file size: {}", err));
                return Err(err_str);
            },
            Ok(segment_size) => segment_size,
        };
        Ok(Config{
            log_path: args[1].clone(),
            port: args[2].clone(),
            segment_size: segment_size,
        })
    }
}