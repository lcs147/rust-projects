use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Parsing arguments error: {err}");
        process::exit(1);
    });

    let result = minigrep::grep(config).unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(1);
    });

    print!("{result}");
}