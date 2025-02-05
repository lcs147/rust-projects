use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("less than 2 arguments in the command line");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = !env::var("IGNORE_CASE").is_ok();
        
        Ok(Config{pattern, file_path, case_sensitive})
    }
}

pub fn grep(Config {pattern, file_path, case_sensitive}: Config) -> Result<String, Box<dyn Error>>{

    println!("pattern: {}, file_path: {}", pattern, file_path);
    let mut pattern = pattern;
    if case_sensitive == false {
        pattern = pattern.to_lowercase();
    }

    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut res = String::new();
    let mut cnt = 1;
    for line in reader.lines() {
        let line = line?;
        if case_sensitive && line.contains(&pattern) || !case_sensitive && line.to_lowercase().contains(&pattern) {
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
    fn case_sensitive() {
        let pattern = String::from("mypat");
        let text = &format!("Lorem ipsum dolor sit amet, consectetur adipiscing elitMYPAT.\nPhasellus venenatis quam sed lacus ultrices, non ultrices tortor dapibus.\nNunc congue, ex.\net sollicitudin lobortis {pattern}\nNulla quis felis sit amet sem blandit rhoncus\nelit nisi finibus{pattern} arcu,\n Nulla quis felis sit amet sem blandit rhoncus\nNulla facilisi. Suspendisse dictum a diam ut rhoncus.\n");
        let correct = &format!("line 4: et sollicitudin lobortis {pattern}\nline 6: elit nisi finibus{pattern} arcu,\n");
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{text}").unwrap(); 
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let result = grep(Config{pattern, file_path, case_sensitive:true}).unwrap();
        assert!(result.as_str() == correct);
    }

    #[test]
    fn case_insensitive() {
        let pattern = String::from("lo");
        let text = &format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nPhasellus venenatis \nLos Angeles\nquam sed lacus ultrices, non ultrices tortor dapibus.\nNunc congue, ex.\net sollicitudin lobortis {pattern}\nNulla quis felis sit amet sem blandit rhoncus\nelit nisi finibus{pattern} arcu,\n Nulla quis felis sit amet sem blandit rhoncus\nNulla facilisi. Suspendisse dictum a diam ut rhoncus.\n");
        let correct = &format!("line 1: Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nline 3: Los Angeles\nline 6: et sollicitudin lobortis lo\nline 8: elit nisi finibuslo arcu,\n");
        
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{text}").unwrap(); 
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let result = grep(Config{pattern, file_path, case_sensitive:false}).unwrap();
        println!("{result}");
        println!("{correct}");
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

        let result = grep(Config{pattern, file_path, case_sensitive:false}).unwrap();
        assert!(result.as_str() == correct);
    }
}
