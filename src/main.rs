#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    buffer = buffer.trim().to_string();
    print!("{}: command not found", buffer);
    Ok(())
}
