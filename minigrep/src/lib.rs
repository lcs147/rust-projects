use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
pub struct Config {
    pub pattern: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("less than 2 arguments in the command line");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        
        return Ok(Config{pattern, file_path});
    }
}

pub fn grep(pattern: &String, file_path: &String) -> Result<String, Box<dyn Error>>{

    println!("pattern: {pattern}, file_path: {file_path}");

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut res = String::new();
    let mut cnt = 1;
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            res += &format!("line {cnt}: {line}\n");
        }
        cnt += 1;
    }

    return Ok(res);
}

#[cfg(test)]
mod tests {
    
    use tempfile::NamedTempFile;
    use std::io::Write;
    use super::*;

    #[test]
    fn should_find_pattern() {
        let pattern = String::from("mypat");
        let text = &format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nPhasellus venenatis quam sed lacus ultrices, non ultrices tortor dapibus.\nNunc congue, ex.\net sollicitudin lobortis {pattern}\nNulla quis felis sit amet sem blandit rhoncus\nelit nisi finibus{pattern} arcu,\n Nulla quis felis sit amet sem blandit rhoncus\nNulla facilisi. Suspendisse dictum a diam ut rhoncus.\n");
        let correct = &format!("line 4: et sollicitudin lobortis {pattern}\nline 6: elit nisi finibus{pattern} arcu,\n");
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{text}").unwrap(); 
        let file_path = temp_file.path().to_string_lossy().into_owned();
        let result = grep(&pattern, &file_path).unwrap();
        assert!(result.as_str() == correct);
    }

    #[test]
    fn shouldnot_find_pattern() {
        let pattern = String::from("mypat");
        let text = &format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n
                                    Phasellus venenatis quam sed lacus ultrices, non ultrices tortor dapibus.\n
                                    Nunc congue, ex.\n
                                    et sollicitudin lobortis\n
                                    Nulla quis felis sit amet sem blandit rhoncus\n
                                    elit nisi finibus arcu,\n
                                    Nulla quis felis sit amet sem blandit rhoncus\n
                                    Nulla facilisi. Suspendisse dictum a diam ut rhoncus.\n");
        let correct = &format!("");
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{text}").unwrap(); 
        let file_path = temp_file.path().to_string_lossy().into_owned();
        let result = grep(&pattern, &file_path).unwrap();

        assert!(result.as_str() == correct);
    }
}
