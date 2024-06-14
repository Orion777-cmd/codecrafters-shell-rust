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
        let input = input.trim().to_string();
        let mut splitted_input = input.split(' ');
        match splitted_input.next().unwrap(){
            "exit" => break,
            "echo" => println!("{}", splitted_input.collect::<Vec<&str>>().join(" ")),
            _ => println!("{}: command not found", input.trim_end())
        }

    }
}
