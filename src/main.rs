extern crate libc;
use std::env;
use std::path::Path;
use std::process::Command;
use std::io;

fn main() {
    println!("RUSH TERMINAL");
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read user command");
        command.pop();
        let mut command_tokens: Vec<&str> = command.split(' ').collect();
        let mut is_background = false;
        if let Some(&"&") = command_tokens.last() {
            is_background = true;
            command_tokens.pop();
        } 

        match command_tokens[0] {
            "exit" => std::process::exit(0),
            "cd" => change_dir(command_tokens[1]),
            _ => execute_command(command_tokens, is_background),
        }
    }
}

fn execute_command(tokens: Vec<&str>, is_background: bool) {
    let mut instance = Command::new(tokens[0]);
    if let Ok(mut child) = instance.args(&tokens[1..]).spawn() {
        if is_background == false {
            child.wait().expect("command was not running");
        } else {
            println!("{} started", child.id());
        }
    } else {
        println!("Command did not start");
    }
}

fn change_dir(new_path: &str) {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(e)  => println!("Failed to change directory\n{}", e),
        _ => {}
    }
}