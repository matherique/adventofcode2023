use std::{fs, io::Result};

pub fn read_file(day: &str) -> Result<String> {
    let filename = format!("./data/{day}.txt");
    let content = fs::read_to_string(filename)?;

    return Ok(content);
}
