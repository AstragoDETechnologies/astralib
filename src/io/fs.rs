use std::{error::Error, io::Write};

#[allow(unused)]
pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename)?;
    Ok(contents)
}

use std::path::Path;
#[allow(unused)]
pub fn write_file(filename: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(Path::new(filename))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
