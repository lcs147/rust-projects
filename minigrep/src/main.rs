use std::env;
use std::error::Error;
use std::process;

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let result = minigrep::grep(&config.pattern, &config.file_path).unwrap();
    print!("{result}");
    
    Ok(())
}