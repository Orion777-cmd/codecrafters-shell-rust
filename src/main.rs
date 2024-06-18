#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;

fn main() {
    
    let mut commands: HashSet<&str> = HashSet::new();
    commands.insert("type");
    commands.insert("echo");
    commands.insert("exit");
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
            "type" => {
                let next_part = splitted_input.next().unwrap();
                if commands.contains(next_part){
                    println!("{} is a shell builtin", next_part.trim_end());
                }else {
                    println!("{}: not found", next_part.trim_end());
                }
            }
            _ => println!("{}: command not found", input.trim_end())
        }

    }
}
