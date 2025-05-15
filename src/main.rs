extern crate libc;
use std::env;
use std::path::Path;
use std::process::Command;
use std::io;
pub mod colors;

fn main() {
    loop {
        print_prompt();
        let mut command_string = String::new();
        io::stdin()
            .read_line(&mut command_string)
            .expect("Failed to read");

        let commands_tokens = tokenize(&mut command_string);
        for mut command_tokens in commands_tokens {
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
}

fn execute_command(tokens: Vec<&str>, is_background: bool) {
    let mut instance = Command::new(tokens[0]);
    if let Ok(mut child) = instance.args(&tokens[1..]).spawn() {
        if is_background == false {
            child.wait().expect("command was not running");
        } else {
            colors::success_logger(format!("{} started", child.id()));
        }
    } else {
        colors::error_logger("command not found".to_string());
    }
}

fn change_dir(new_path: &str) {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(e)  => println!("Failed to change directory\n{}", e),
        _ => {}
    }
}

fn print_prompt() {
    let path = env::current_dir().unwrap(); 
    println!("{}{} >> RUSHING IN {}{}", colors::ANSI_BOLD, colors::ANSI_COLOR_CYAN, path.display(), colors::RESET);
}

fn tokenize(command_string: &mut String) -> Vec<Vec<&str>> {
    command_string.pop();
    let commands: Vec<&str> = command_string.split(',').collect();
    let mut command_tokens: Vec<Vec<&str>> = Vec::new();
    for command in commands.iter() {
        command_tokens.push(command.split(' ').collect());
    }
    command_tokens
}
