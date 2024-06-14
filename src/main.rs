#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str(){
            "exit 0" => break,
            _ => println!("{}: command not found", input.trim())
        }

    }
}
