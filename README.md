# Ironman

Ironman is a practical, extensible Cargo workspace designed to help software engineers learn Rust by building something real, studying data structures, and practicing language fundamentals. The goal is education: understand Rust while creating small, usable, and well-organized code.

## Why Ironman?
- Learn Rust by doing: build a CLI step by step.
- Practice data structures with proper unit/integration tests.
- Study language fundamentals (variables, structs, traits, formatting, etc.) in isolated, runnable examples.
- Keep concerns separated as the project grows, instead of piling everything into one crate.

## Project Structure

This is a Cargo workspace with three crates under `crates/`:

```
ironman/
├── Cargo.toml                 workspace manifest (members = ["crates/*"])
└── crates/
    ├── explorer/               bin "ironman" — the actual CLI file explorer
    │   └── src/
    │       ├── main.rs
    │       ├── explorer.rs
    │       └── repl.rs
    ├── data_structures/        lib — data structures with tests (binary tree, and more to come)
    │   ├── src/
    │   │   └── binarytree/
    │   └── tests/
    └── fundamentals/           lib + examples/ — Rust language fundamentals studies
        └── examples/
            ├── variables.rs
            ├── structs.rs
            ├── arrays.rs
            ├── formatting.rs
            ├── debug_mode.rs
            ├── functions.rs
            ├── tuple.rs
            └── traits.rs
```

- **`explorer`**: the real app. A simple File Explorer REPL in your terminal. Implement and use basic commands like `ls`, `cd`, `pwd`, and more as you grow the project. Not meant to compete with full-featured file managers — it's a learning vehicle.
- **`data_structures`**: a library crate for studying data structures (currently a binary search tree). Each structure should come with its own unit and/or integration tests.
- **`fundamentals`**: a collection of small, independent Rust scripts using [Cargo examples](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples) to practice language fundamentals — variables, structs, tuples, traits, formatting, debug printing, functions, and arrays.

## Quick Start

### Prerequisites
- Rust toolchain installed (via rustup): https://www.rust-lang.org/learn/get-started

### Build everything
```bash
cargo build --workspace
```

### Run the explorer CLI
```bash
cargo run -p explorer
```

Inside the REPL:
```
ironman cli> pwd
ironman cli> exit
```

Optionally install the binary locally:
```bash
cargo install --path crates/explorer
ironman
```

### Run a fundamentals example
```bash
cargo run --example variables -p fundamentals
cargo run --example traits -p fundamentals
```

### Run data structures tests
```bash
cargo test -p data_structures
```

### Run everything (build + tests across the workspace)
```bash
cargo build --workspace
cargo test --workspace
```

## Learn by Building

Suggested learning path while coding:
- **explorer**: parse REPL commands, work with paths and the filesystem (list, change directory, read files), handle errors clearly, add tests for command behavior.
- **data_structures**: implement a new structure (linked list, stack, queue, heap...), write unit tests inside the module and integration tests under `tests/`.
- **fundamentals**: add a new example per concept you want to study; keep each one runnable on its own via `cargo run --example <name> -p fundamentals`.

## Extending Ironman

Ideas to implement next in `explorer`:
- `ls`: wire up `Explorer::list_directory` to a REPL command.
- `cd <path>`: wire up `Explorer::change_directory` to a REPL command.
- `cat <file>`: show file contents.
- `mkdir <name>`: create a folder.
- Flags (e.g., `ls -a`, `ls -l`).

Ideas to implement next in `data_structures`:
- Linked list, stack, queue, hash map from scratch.
- Sorting and searching algorithms.

Implementation tips:
- Add a dispatcher in `crates/explorer/src/repl.rs` that matches the command name.
- Put command logic in dedicated modules/functions.
- Keep output simple and predictable.

## Roadmap

- Core explorer commands: `ls`, `cd`, `pwd`.
- Better error messages and logging.
- More data structures with full test coverage.
- More fundamentals examples (generics, lifetimes, error handling, iterators, async).
- Cross-platform checks (Windows, macOS, Linux).

## Contributing

Contributions are welcome, especially improvements that help learners:
- Keep code clear and well-structured.
- Prefer small, focused pull requests.
- Add examples and docs when you introduce new commands or structures.

## Disclaimer

Ironman is an educational project. It is not intended to replace mature file explorer tools in the community. Use it to learn Rust, data structures, and project organization, and extend it as your skills grow.
