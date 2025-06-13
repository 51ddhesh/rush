extern crate libc;
// Standard library imports for file system, process handling, and I/O operations
use std::env;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
pub mod colors;

fn main() {
    // Ignore interrupt signals (Ctrl+C and Ctrl+\) in the parent shell process
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }

    // Track the exit status of the last command
    let mut last_exit_status = true;

    // Main shell loop
    loop {
        // Display the shell prompt with current directory and status
        print_prompt(last_exit_status);
        let mut command_string = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush the buffer");

        // Read user input
        io::stdin()
            .read_line(&mut command_string)
            .expect("Failed to read");

        // Parse the input into command tokens
        let commands_tokens = tokenize(&mut command_string);
        for mut command_tokens in commands_tokens {
            // Check if command should run in background
            let mut is_background = false;
            if let Some(&"&") = command_tokens.last() {
                is_background = true;
                command_tokens.pop();
            }
            // Handle built-in commands or execute external commands
            match command_tokens[0] {
                "exit" => std::process::exit(0),
                "cd" => {
                    last_exit_status = change_dir(command_tokens[1]);
                }  
                _ => {
                    last_exit_status = execute_command(command_tokens, is_background);
                }
            }
        }
    }
}

/// Executes a command with the given tokens and background flag
/// Returns true if the command executed successfully, false otherwise
#[allow(unused_unsafe)]
fn execute_command(tokens: Vec<&str>, is_background: bool) -> bool {
    if tokens.is_empty() {
        colors::error_logger("No command entered".to_string());
        return false;
    }
    let mut instance = Command::new(tokens[0]);
    // Spawn the command process with signal handling
    if let Ok(mut child) = unsafe {
        instance.args(&tokens[1..])
            .pre_exec(|| {
                unsafe {
                    // Restore default signal handling in child process
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                }
                Ok(())
            })
            .spawn()
    } {
        if is_background == false {
            // Wait for foreground process to complete
            return child.wait().expect("Command was not running").success();
        } else {
            // Print process ID for background process
            colors::success_logger(format!("{} started", child.id()));
            true
        }
    } else {
        colors::error_logger("Command not found".to_string());
        false
    }
}

/// Changes the current working directory to the specified path
/// Returns true if successful, false otherwise
fn change_dir(new_path: &str) -> bool {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(e)  => {
            colors::error_logger(format!("Failed to change directory\n{}", e));
            return false;
        },
        _ => {},
    }
    return true;
}

/// Prints the shell prompt with the current directory and last command status
/// Uses color coding: green arrow for success, red for failure
fn print_prompt(last_exit_status: bool) {
    let path = env::current_dir().unwrap();
    println!("{}RUSHING IN {}{}{}", colors::ANSI_BOLD, colors::ANSI_COLOR_CYAN, path.display(), colors::RESET);
    if last_exit_status {
        print!("{}\u{2ba1}{}  ", colors::GREEN, colors::RESET);
    } else {
        print!("{}\u{2ba1}{}  ", colors::RED, colors::RESET);
    }
}

/// Tokenizes the input command string into a vector of command token vectors
/// Splits commands by commas and then splits each command by spaces
fn tokenize(command_string: &mut String) -> Vec<Vec<&str>> {
    command_string.pop();
    let commands: Vec<&str> = command_string.split(',').collect();
    let mut command_tokens: Vec<Vec<&str>> = Vec::new();
    for command in commands.iter() {
        command_tokens.push(command.split(' ').collect());
    }
    command_tokens
}
