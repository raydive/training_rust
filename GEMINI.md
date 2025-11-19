# Project Overview

This repository contains a collection of Rust projects created while following "The Rust Programming Language" book. The projects are organized into directories corresponding to the chapters of the book. Each chapter directory contains one or more individual Rust projects, each with its own `Cargo.toml` file.

The purpose of this repository is for learning and practicing Rust. The code examples range from simple "Hello, World!" programs to more complex applications like a command-line grep tool.

## Key Projects

*   **`guessing_game`**: A simple game where the user has to guess a number. This project is a good introduction to basic Rust concepts like variables, I/O, and control flow.
*   **`chapter12/minigrep`**: A simplified version of the `grep` command-line tool. This project demonstrates more advanced concepts like file I/O, error handling, and command-line argument parsing.

# Building and Running

Each project in this repository is a standalone Rust project that can be built and run using Cargo, the Rust build tool and package manager.

To build a project, navigate to the project's directory (the one containing the `Cargo.toml` file) and run:

```bash
cargo build
```

To run a project, use the `cargo run` command from the project's directory:

```bash
cargo run
```

For projects that accept command-line arguments, like `minigrep`, you can pass them after a `--` separator:

```bash
cargo run -- <query> <filename>
```

To run the tests for a project, use the `cargo test` command:

```bash
cargo test
```

# Development Conventions

*   The code follows the standard Rust formatting and style guidelines, which can be enforced using `rustfmt`.
*   Each project is self-contained and has its own dependencies defined in its `Cargo.toml` file.
*   The code is organized into modules and functions to promote code reuse and maintainability.
