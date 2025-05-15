# Rush - A Simple Unix Shell in Rust

Rush is a lightweight Unix shell implementation written in Rust. It provides basic shell functionality with a clean and modern interface.

## Features

- Command execution with foreground and background process support
- Built-in `cd` command for directory navigation
- Command chaining using commas (`,`)
- Background process execution using `&`
- Colorful prompt showing current directory and command status

## Usage

### Building from Source

1. Make sure you have Rust and Cargo installed on your system
```bash
rustc --version
cargo --version
```
2. Clone this repository
```bash
git clone https://github.com/51ddhesh/rush.git
```
3. Build the project:
```bash
cargo build --release
```
4. Run the shell:
```bash
cargo run
```

### Shell Features

- The prompt shows your current directory in cyan
- Command status is indicated by the arrow color:
  - Green: Last command succeeded
  - Red: Last command failed
- Run multiple commands by separating them with commas:
```bash
ls, pwd, echo "hello"
```
- Run commands in background by adding `&`:
```bash
sleep 10 &
```
- Use `cd` to change directories
- Use `exit` to quit the shell

## Dependencies

- <code>rustc >= 1.85.0</code>
- <code>libc = 0.2</code> - For Unix system calls and signal handling

## License

This project is open source and available under the MIT License. 