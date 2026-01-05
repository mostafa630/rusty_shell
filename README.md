# Rust Shell Implementation

A POSIX-compliant shell implementation built in Rust as part of the CodeCrafters "Build Your Own Shell" challenge. This shell supports command parsing, execution, I/O redirection, pipelines, and interactive features like auto-completion and command history.

## Features

### Core Shell Functionality
- **Interactive REPL** with readline support
- **Command parsing** with proper tokenization
- **Built-in commands**: `exit`, `echo`, `type`, `pwd`, `cd`, `clear`
- **External program execution** with PATH resolution
- **Background process execution** using `&`
- **Auto-completion** for commands
- **Command history** with arrow key navigation

### I/O Redirection Support
- **Input redirection**: `command < file`
- **Output redirection**: `command > file` (truncate)
- **Output append**: `command >> file`
- **Error redirection**: `command 2> file`
- **Error append**: `command 2>> file`

### Advanced Features
- **Pipeline support**: `command1 | command2`
- **Quote handling**: Single and double quotes with escape sequences
- **Background execution**: `command &`
- **Proper error handling** and exit codes

## Project Structure

```
src/
├── main.rs           # Entry point and REPL loop
├── lib.rs            # Module declarations
├── tokenizer.rs      # Lexical analysis and tokenization
├── parser.rs         # Command parsing and AST generation
├── executor.rs       # Command execution and I/O handling
├── command.rs        # Built-in and external command implementations
└── auto_complete.rs  # Tab completion functionality
```

### Architecture Overview

The shell follows a clean pipeline architecture:

1. **Tokenizer** (`tokenizer.rs`): Converts input strings into tokens
2. **Parser** (`parser.rs`): Transforms tokens into structured commands
3. **Executor** (`executor.rs`): Executes parsed commands with proper I/O handling
4. **Command** (`command.rs`): Implements built-in commands and external program execution

## Usage Examples

### Basic Commands
```bash
$ echo "Hello, World!"
Hello, World!

$ pwd
/home/user/project

$ cd /tmp
$ pwd
/tmp
```

### I/O Redirection
```bash
# Output redirection
$ echo "Hello" > output.txt
$ cat output.txt
Hello

# Append to file
$ echo "World" >> output.txt
$ cat output.txt
Hello
World

# Error redirection
$ ls nonexistent 2> error.log
$ cat error.log
ls: cannot access 'nonexistent': No such file or directory
```

### Pipeline Operations
```bash
# Basic pipeline
$ ls -la | grep ".rs"
-rw-r--r-- 1 user user  1234 Jan  5 10:00 main.rs
-rw-r--r-- 1 user user   567 Jan  5 10:00 parser.rs

# Complex pipeline with redirection
$ cat file.txt | grep "pattern" > results.txt
```

### Background Execution
```bash
# Run command in background
$ sleep 10 &
$ echo "This runs immediately"
This runs immediately
```

### Built-in Commands
```bash
# Check command type
$ type echo
echo is a shell builtin

$ type ls
ls is /bin/ls

# Exit with code
$ exit 0
```

## Getting Started

### Prerequisites
- Rust 1.80+ installed
- Cargo package manager

### Building and Running

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd codecrafters-shell
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the shell**:
   ```bash
   ./your_program.sh
   # or directly with cargo
   cargo run
   ```

4. **Interactive usage**:
   ```bash
   $ echo "Welcome to Rust Shell!"
   Welcome to Rust Shell!
   $ pwd
   /current/directory
   $ exit 0
   ```

## Implementation Details

### Tokenization
The tokenizer handles:
- Word boundaries and whitespace
- Quote parsing (single and double quotes)
- Special characters (`|`, `&`, `>`, `<`, `>>`, `2>`, `2>>`)
- Escape sequences with backslashes

### Command Parsing
The parser creates a structured representation:
- Commands with arguments
- I/O redirections
- Pipeline chains
- Execution modes (foreground/background)

### Execution Engine
The executor manages:
- Built-in command dispatch
- External program execution via PATH lookup
- File descriptor management for I/O redirection
- Process spawning and management
