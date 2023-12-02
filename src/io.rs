//! This module is used to perform input and output operations.

pub mod fs;

use std::{error::Error, io::Write};

/// Prompts for an input on `stdin` and returns it in a result.
#[allow(unused)]
pub fn input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    match std::io::stdout().flush() {
        Err(_) => {
            println!("");
        }
        _ => (),
    }
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    Ok(input)
}
