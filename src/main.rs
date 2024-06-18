#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::env;
// use std::path::Path;

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
                } else {
                    let path_var = env::var_os("PATH").unwrap();
                    let paths: Vec<_> = env::split_paths(&path_var).collect();
                    let mut found = false;
                    for path in paths {
                        let command_path = path.join(next_part);
                        if command_path.exists() {
                            println!("{} is {}", next_part, command_path.display());
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("{}: not found", next_part.trim_end());
                    }
                }
            }
            _ => println!("{}: command not found", input.trim_end())
        }

    }
}
