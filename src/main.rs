#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    
    let mut commands: HashSet<&str> = HashSet::new();
    commands.insert("type");
    commands.insert("echo");
    commands.insert("exit");
    commands.insert("pwd");
    commands.insert("cd");
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
            },
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            },
            "cd" => {
                let next_part = splitted_input.next().unwrap();
                let path = PathBuf::from(next_part);
                if path.exists() {
                    env::set_current_dir(&path).unwrap();
                } else {
                    println!("cd: {}: No such file or directory", next_part);
                }
            },
            command_str => {
                let path_var = env::var_os("PATH").unwrap();
                let paths: Vec<_> = env::split_paths(&path_var).collect();
                let mut found = false;
                let mut command_path = PathBuf::new();
                let args: Vec<String> = splitted_input.map(|s| s.to_string()).collect();
                for path in &paths {
                    let potential_path = path.join(command_str);
                    if potential_path.exists() {
                        found = true;
                        command_path = potential_path.clone();
                        break;
                    }
                }
                if found {
                    let output = Command::new(command_path)
                        .args(&args)
                        .output()
                        .expect("Failed to execute command");
                    io::stdout().write_all(&output.stdout).unwrap();
                } else {
                    println!("{}: command not found", command_str.trim_end());
                }
            },
        }

    }
}
