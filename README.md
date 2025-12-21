# Ironman

Ironman is a practical, extensible CLI project designed to help software engineers learn Rust by building something real. The goal is education: understand Rust fundamentals while creating a small, usable command-line tool.

Ironman acts as a simple File Explorer in your terminal. You will implement and use basic commands like `ls`, `cd`, and more as you grow the project. This is not meant to compete with full-featured file managers; it is a learning vehicle.

## Why Ironman?
- Learn Rust by doing: build a CLI step by step.
- Keep scope small and focused on core concepts.
- Extend at your own pace with new commands and features.
- Readable, approachable code intended for learners.

## Quick Start

### Prerequisites
- Rust toolchain installed (via rustup): https://www.rust-lang.org/learn/get-started

### Build
```bash
cargo build
```

### Run
Run Ironman through Cargo while you develop:
```bash
cargo run -- ls
cargo run -- cd .
cargo run -- help
```

Optionally install the binary locally:
```bash
cargo install --path .
ironman ls
```

## Usage

Ironman aims to support simple file exploration commands you typically use from a shell. As you implement features, try them like:

- List directory contents:
	```bash
	cargo run -- ls
	```

- Change directory:
	```bash
	cargo run -- cd <path>
	```

- Show help:
	```bash
	cargo run -- help
	```

Note: Command availability depends on what you have implemented so far. Start small, then add more.

## Project Structure

- `src/main.rs`: CLI entry point; parse args, route to commands.
- Other Rust files (e.g., `variables.rs`, `structs.rs`, `arrays.rs`) can be used to practice Rust fundamentals and organize features.
- You can split commands into modules to keep code clean as the project grows.

## Learn by Building

Suggested learning path while coding:
- Parse command-line arguments.
- Work with paths and the filesystem (list, change directory, read files).
- Handle errors clearly and return results.
- Add tests for command behavior.
- Refactor into modules for readability.

## Extending Ironman

Ideas to implement next:
- `pwd`: print current directory.
- `cat <file>`: show file contents.
- `mkdir <name>`: create a folder.
- Flags (e.g., `ls -a`, `ls -l`).

Implementation tips:
- Add a dispatcher in `src/main.rs` that matches the first argument (command name).
- Put command logic in dedicated modules/functions.
- Keep output simple and predictable.

## Roadmap

- Core commands: `ls`, `cd`, `pwd`.
- Better error messages and logging.
- Basic tests for each command.
- Optional formatting and color output.
- Cross-platform checks (Windows, macOS, Linux).

## Contributing

Contributions are welcome, especially improvements that help learners:
- Keep code clear and well-structured.
- Prefer small, focused pull requests.
- Add examples and docs when you introduce new commands.

## Disclaimer

Ironman is an educational project. It is not intended to replace mature file explorer tools in the community. Use it to learn Rust and CLI design, and extend it as your skills grow.
# ironman
