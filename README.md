# Learning Rust

My personal collection of Rust exercises and notes — the files I use to learn
the language, mainly following *The Rust Programming Language* (the official
Rust book).

Every topic gets its own folder so it can be built, run, and experimented with
independently.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install) installed
  (`rustup`, which includes `cargo`)

```bash
rustc --version
cargo --version
```

## Usage

Each folder is an independent Cargo project, so `cd` into the one you want to
work on:

```bash
cd <project-folder>
cargo run
```

Useful commands:

```bash
cargo check      # fast type/compile check without building
cargo build      # compile (debug), binary lands in target/
cargo run        # build and run
cargo fmt        # format the code
cargo clippy     # lint suggestions
```

To run everything at once:

```bash
for dir in */; do (cd "$dir" && cargo run); done
```

## Adding a new topic

Create a fresh project with:

```bash
cargo new <topic-folder>
```

Then write the smallest program that demonstrates the concept — a few lines
that compile, print something, and prove the behaviour (often with
`assert_eq!`). Small and focused beats big and half-finished.

## Topics to work through

- Variables, mutability, and shadowing
- Data types: integers, floats, booleans, characters
- Control flow: `if`, `loop`, `while`, `for`
- Functions: parameters, return values
- Ownership, borrowing, and references
- Structs, enums, and pattern matching
- Error handling with `Option` and `Result`
- Collections: `Vec`, `String`, `HashMap`
- Modules, generics, traits, and tests with `#[test]`

## Notes

- Projects use Rust edition 2024.
- `target/` holds build output and can be deleted at any time.
