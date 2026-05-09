# minigrep

A minimal command-line search tool built in Rust. Searches a text file for lines that match a query string, with optional case-insensitive matching via an environment variable.

This project is the worked example from **Chapter 12: An I/O Project: Building a Command Line Program** of [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, Rust 1.85+)

## Build

```sh
cargo build
```

For a release build:

```sh
cargo build --release
```

## Run

```sh
cargo run -- <query> <file>
```

**Example** — search for lines containing `"the"` in `poem.txt`:

```sh
cargo run -- the poem.txt
```

### Case-insensitive search

Set the `IGNORE_CASE` environment variable to any value to match regardless of letter case:

```sh
IGNORE_CASE=1 cargo run -- THE poem.txt
```

On Windows (PowerShell):

```powershell
$env:IGNORE_CASE=1; cargo run -- THE poem.txt
```

### Redirecting output

Matched lines are written to stdout; error messages go to stderr, so you can redirect them independently:

```sh
cargo run -- the poem.txt > output.txt
```

## Tests

```sh
cargo test
```

The test suite (in `src/lib.rs`) covers:

| Test | What it checks |
|---|---|
| `case_sensitive` | Returns only lines that match the query with the same casing |
| `case_insensitive` | Returns lines that match the query regardless of casing |

## Project structure

```
minigrep/
├── src/
│   ├── main.rs   # Entry point: argument parsing, Config struct, run function
│   └── lib.rs    # Core logic: search_case_sensitive, search_case_insensitive
├── poem.txt      # Sample input file
└── Cargo.toml
```

### Key design points

**`src/main.rs`**

- `Config::build` validates that at least two arguments (query + file path) were supplied and reads the `IGNORE_CASE` environment variable. It returns a `Result` so the caller can handle the error gracefully instead of panicking.
- `run` reads the file and dispatches to the appropriate search function. It returns `Result<(), Box<dyn Error>>` so any I/O error propagates with `?` rather than unwrapping.
- `main` calls both functions and exits with a non-zero status code on any failure, printing the error to stderr with `eprintln!`.

**`src/lib.rs`**

- `search_case_sensitive` iterates over lines and collects those that contain the query.
- `search_case_insensitive` lower-cases both the line and the query before comparing, then returns the original (un-lowercased) line.
- Both functions use explicit lifetime annotations (`'a`) to tell the compiler that the returned string slices borrow from `contents`, not from `query`.

## Concepts covered

- Accepting and parsing command-line arguments (`std::env::args`)
- Reading a file to a `String` (`std::fs::read_to_string`)
- Separating concerns between a binary crate (`main.rs`) and a library crate (`lib.rs`)
- Returning and propagating errors with `Result` and the `?` operator
- Writing errors to stderr (`eprintln!`) vs. results to stdout (`println!`)
- Using environment variables (`std::env::var`) for runtime configuration
- Lifetime annotations on function signatures
- Writing unit tests with `#[cfg(test)]`
