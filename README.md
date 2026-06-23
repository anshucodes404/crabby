# 🦀 Crabby - A Rust Shell Emulator

A lightweight, interactive shell emulator written in **Rust**. Crabby brings the power of command-line execution directly to your terminal with a custom shell interface. Built to demonstrate Rust fundamentals including I/O operations, string processing, and system command execution.

---

## ✨ Features

- **Interactive Command Prompt**: Custom shell prompt with a friendly crab emoji (`crabby🦀> `)
- **Command Execution**: Execute any system command directly from the shell
- **Directory Navigation**: Built-in `cd` command for seamless directory traversal
- **Clean Exit**: Graceful shutdown with the `exit` command
- **Error Handling**: Robust error messages for failed commands or invalid operations
- **Input Processing**: Intelligent whitespace-based command parsing

---

## 🚀 Getting Started

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- Any operating system with shell access (Linux, macOS, Windows with WSL/Git Bash)

### Installation

Clone the repository and navigate to the project directory:

```bash
cd rust_projects/crabby
```

### Running the Project

Start the shell emulator:

```bash
cargo run
```

You'll see the interactive prompt:

```
crabby🦀> 
```

---

## 📖 Usage Guide

### Basic Commands

Once inside the Crabby shell, you can:

1. **Execute System Commands**
   ```
   crabby🦀> ls
   crabby🦀> pwd
   crabby🦀> echo "Hello from Crabby!"
   ```

2. **Navigate Directories**
   ```
   crabby🦀> cd /path/to/directory
   crabby🦀> cd ..
   crabby🦀> cd ~
   ```

3. **Exit the Shell**
   ```
   crabby🦀> exit
   EXITTING CRABBY...
   ```

### Command Examples

```bash
# View current directory
crabby🦀> pwd

# List files
crabby🦀> ls -la

# Create a file
crabby🦀> touch myfile.txt

# Print text
crabby🦀> echo "Rust is awesome!"

# Change to home directory
crabby🦀> cd ~

# Exit the shell
crabby🦀> exit
```

---

## 🏗️ Project Structure

```
crabby/
├── Cargo.toml          # Project manifest and dependencies
├── README.md           # This file
└── src/
    ├── main.rs         # Main shell loop and command routing
    └── fns.rs          # Core shell functions
```

### Core Components

#### `src/main.rs`
The heart of Crabby's operation:
- **Main Event Loop**: Continuously reads and processes user input
- **Command Router**: Matches commands to appropriate handlers
- **Exit Handler**: Gracefully terminates the shell

#### `src/fns.rs`
Essential shell functions:
- **`read_input()`**: Prompts user for input and reads from stdin
- **`process_input()`**: Parses input into command tokens
- **`handle_cd()`**: Manages directory changes
- **`process_cmd()`**: Executes external system commands

---

## 🔍 How It Works

1. **Startup**: Program starts with an interactive prompt ready for input
2. **Input Loop**: Continuously waits for user commands
3. **Parsing**: Input is tokenized by whitespace into command and arguments
4. **Routing**:
   - `exit` → Terminate the shell
   - `cd` → Change current working directory
   - Any other command → Execute as a system command
5. **Execution**: Commands run with full output/error reporting
6. **Cleanup**: Proper error handling and graceful shutdown

---

## 📚 Learning Value

This project is an excellent demonstration of:

- **Rust I/O Operations**: Reading from stdin and writing to stdout
- **String Manipulation**: Parsing and processing input strings
- **Process Management**: Spawning and managing child processes
- **Error Handling**: Robust error reporting and recovery
- **Module Organization**: Clean code structure with separated concerns
- **Loops and Pattern Matching**: Idiomatic Rust control flow

---

## 🛠️ Building for Release

Create an optimized production build:

```bash
cargo build --release
```

The compiled binary will be available at:
```
target/release/crabby
```

---

## 🧪 Compilation

Verify the code compiles without warnings:

```bash
cargo check
cargo clippy
```

---

## 📝 Notes

- Commands are executed in the current working directory context
- The `cd` command modifies the shell's working directory (not just the child process)
- All system commands inherit the shell's environment
- Empty input is gracefully skipped without errors

---

## 🎓 Future Enhancements

Potential features for expansion:

- [ ] Command history with arrow key navigation
- [ ] Tab completion for commands and file paths
- [ ] Pipe operator (`|`) support
- [ ] Output redirection (`>`, `>>`, `<`)
- [ ] Background job execution (`&`)
- [ ] Custom aliases and shortcuts
- [ ] Configuration file support
- [ ] Script execution capability

---

## 📄 License

This project is open source and available for educational purposes.

---

## 👨‍💻 Contributing

Feel free to fork, modify, and improve Crabby! This is a great learning project for Rust enthusiasts.

---

**Happy coding! 🦀**
