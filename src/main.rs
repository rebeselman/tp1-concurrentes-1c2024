use std::env;
use std::process;
use tp1::config::Config;
use tp1::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Number of threads {}", config.number_of_threads);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_number_worker_threads() -> Result<(), std::io::Error> {
        let vec = vec!["/hi".to_string(), "0".to_string()];
        let config = Config::build(&vec);
        match config {
            Ok(_) => {
                panic!("Zero is not a valid number of worker threads")
            }
            Err(err) => {
                assert_eq!(err, "Invalid number of worker threads")
            }
        }
        Ok(())
    }
}
