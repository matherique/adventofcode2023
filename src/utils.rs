use std::{fs, env, io::Result};

pub fn read_file(day: &str) -> Result<String> {
    let example = env::var("EXAMPLE").unwrap_or_else(|_| String::from("false"));

    let mut filename = format!("./data/{day}.txt");
    if example == "true" {
        filename = format!("./data/{day}.example.txt");
    }

    let content = fs::read_to_string(filename)?;
    
    return Ok(content);
}
