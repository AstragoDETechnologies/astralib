//! This module is used to perform input and output operations.

use tracing::*;

pub mod fs;

use std::{error::Error, io::Write};

/// Prompts for an input on `stdin` and returns it in a result.
#[allow(unused)]
pub fn input(prompt: &str) -> Result<String, Box<dyn Error>> {
    debug!("Printing prompt.");
    print!("{}", prompt);
    match std::io::stdout().flush() {
        Err(_) => {
            error!("Could not flush stdout buffer. Printing newline.");
            println!("");
        }
        _ => (),
    }
    debug!("Reading input to String...");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    debug!("Trimming input.");
    input = input.trim().to_string();
    Ok(input)
}
