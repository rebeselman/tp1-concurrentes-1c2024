//! #Config
//! This is an object which function is to parse the input of the fork api.

/// Config struct.
/// Contains the desired number of worker threads to process the data.
pub struct Config {
    pub number_of_threads: usize,
}

impl Config {
    /// Method to parse the number of worker threads.
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.is_empty() {
            return Err("not enough arguments".to_owned());
        }
        let number_of_threads = args[1]
            .parse::<usize>()
            .map_err(|error| error.to_string())?;
        if number_of_threads == 0 {
            return Err("Invalid number of worker threads".to_string());
        }
        Ok(Config { number_of_threads })
    }
}
