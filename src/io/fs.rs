//! This submodule is used for io operations on the file system.

use tracing::*;

use std::path::Path;
use std::{error::Error, io::Write};

/// Reads a file as String.
pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    debug!("Reading contents of {} to String", path);
    let contents: String = std::fs::read_to_string(path)?;
    Ok(contents)
}

/// Writes a file as String.
pub fn write_file(path: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing &str contents to {}", path);
    let mut file = std::fs::File::create(Path::new(path))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
