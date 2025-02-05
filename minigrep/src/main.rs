use std::env;
use std::error::Error;

use minigrep::grep;
fn main() -> Result<(), Box<dyn Error>> {
    let pattern = env::args().nth(1).expect("Missing 1st argument!");
    let file_path = env::args().nth(2).expect("Missing 2nd argument!");

    let result = grep(&pattern, &file_path).unwrap();
    print!("{result}");
    
    Ok(())
}
