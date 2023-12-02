pub mod fs;

use std::{error::Error, io::Write};
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
