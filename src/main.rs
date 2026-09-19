#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.trim();
        if buffer == "exit" {
            return Ok(());
        }
        println!("{}: command not found", buffer);
    }
}
