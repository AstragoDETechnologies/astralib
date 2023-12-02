//! This submodule is used for io operations on the file system.

use std::{error::Error, io::Write};

/// Reads a file as String.
#[allow(unused)]
pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(path)?;
    Ok(contents)
}

/// Writes a files as String.
use std::path::Path;
#[allow(unused)]
pub fn write_file(path: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::create(Path::new(path))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
