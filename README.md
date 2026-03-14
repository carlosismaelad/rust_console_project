# Rust Masterclass

`rust_console_project` is an interactive command-line project created while studying **The Rust Programming Language**.  
Its main goal is to help beginners get comfortable with Rust syntax and core concepts through small, focused examples.

## Project Goal

This project is designed for people who are starting with Rust and want to practice the language fundamentals in a practical way.  
Instead of reading only theory, you can run the app, open each menu, and see examples executing in real time.

## What You Can Learn Here

The project is organized into learning modules:

- **Fundamentals**
	- First Rust program
	- Immutable and mutable variables
	- Constants and shadowing
	- Arithmetic, relational, and logical operators

- **Types**
	- Basic scalar and string types (`bool`, `char`, `&str`, `String`, integers, floats)
	- Sequences (`tuple`, `array`, `slice`, `Vec`)
	- Custom types (`struct`, `enum`)

- **Conditional Structures**
	- `if / else if / else`
	- `for` loops (range, reverse, step)
	- Iterating arrays
	- `while` and `loop`

- **Functions**
	- Function declaration and parameters
	- Return values
	- Closures with practical examples (`map`, `filter`)

- **Ownership**
	- Scope and lifetime basics
	- Move, clone, and copy behavior
	- Ownership transfer between functions
	- Immutable and mutable references
	- Dangling references
	- String slices

## How the Application Works

When you run the program, you get a terminal menu:

1. Choose a main topic (Fundamentals, Types, Conditional Structures, Functions, Ownership).
2. Choose a specific example inside that topic.
3. Read the output and return to the menu to continue learning.

This makes the project useful as a guided, hands-on Rust playground.

## Project Structure

```text
src/
	main.rs
	fundamentals/
	types/
	conditional_structures/
	functions/
	ownership/
	utils/
```

- `main.rs`: entry point and top-level navigation menu.
- Each module folder contains examples related to that topic.
- `utils/terminal.rs`: helpers for console UI (clear screen, menu, wait for input).

## Requirements

- Rust toolchain installed (`rustup` + `cargo`)

Check installation:

```bash
rustc --version
cargo --version
```

## Running the Project

From the project root:

```bash
cargo run
```

## Notes

- The code is educational and intentionally simple.
- You can use this project as a base to add new modules and examples as you progress.
