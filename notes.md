# The Rust Programming Language
(No Starch Press)

## Chapter 01 - Getting Started

### Summary

----------------------------------------
#### Helpful commands

- Update Rust
    - `$ rustup update`
- Uninstall Rust
    - `$ rustup self uninstall`
- Check version
    - `$ rustc --version`
- View offline docs
    - `$ rustup doc`
- Make new program with Cargo
    - `$ cargo new hello_cargo`
- Build a Cargo project
    - `$ cargo build`
- Build and run a cargo project
    - `$ cargo run`
- Check to see if a project will build
    - `$ cargo check`
- Build project for release
    - `$ cargo build --release`

#### Conventions

- Formatting can be standardized using `rustfmt`
- `fn() {` function name, then space, then brace
- Indents are four spaces, not a tab
----------------------------------------

### Notes

Tools installed with Rust
- **Cargo** - dependency manager and build tool
- **Rustup** - update manager
- **rustc** - compile tool
- **Rustfmt** - Linter
- **Rust Language Server** - For IDE code completion, syntax highlighting, error messages, etc

Anatomy of a Rust program
- `fn main() {}` The main() function is always the first code that runs
- `macro!()` macros are indicated by `!`
- Lines end with `;`
- After compiling, three files are made
    - `main.rs`- source code
    - `main.exe` - executable
    - `main.pdb` - file containing debugging information (also a binary file)

This book is using Rust edition **2018**!

Cargo expects source files to be in the *src* directory, and only configuration files, README files, license information, etc. to be in the top-level directory.

After building Cargo project
- Executable in *target/debug/hello_world.exe*. This is because Cargo builds to **debug** by default, but `cargo build --release` would place the executable in *target/**release**/hello_world.exe*
- *Cargo.lock* at top level; this tracks dependencies

Cargo only rebuilds changed files

Building to release allows you to build with optimizations. These optimizations take longer to compile, but they make the program run faster. 

Because of Cargo, you can easily run a git repository:
```sh
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

## Chapter 02 - Programming a Guessing Game

### Summary

----------------------------------------

#### Language Features

- `Result` type is an enumeration of `[Ok, Err]`
    - `Ok` is a successful return. Inside of the `Ok` is the successful value.
    - `Err` is an error return. Inside of the `Err` is information about the error.
    - There are many `Result` types. There's a generic `Result` in the standard library, and also specific versions for submodules, such as `io::Result`
    - Each `Result` type has methods defined on it, for example `io::Result` has an `.expect()` method which will crash the program if the `Result` returns an `Err`.
    - The right way to use `Result` is to handle both `Ok` and `Err`. Using `.expect()` is only for development.
- A **crate** is a collection of Rust source code files
    - Crates like `guessing_game` meant to be compiled and run are *binary crates*
    - Crates like `rand` meant to be used in other programs are *library crates*.
- `Cargo.lock` is a file created that contains all of the working dependencies for a project. Future builds will use these dependencies, giving you a reproducible build.
- `$ cargo update` will figure out the latest versions that work with your `Cargo.toml` configuration, creating a new `Cargo.lock`

#### Crates

- `std` - Standard library. Contains many basic features.
- `std::io` - i/o crate in the standard library. Contains things like `stdin()` for reading user input 
- `rand` - Random generation library

#### Functions

| Name | Signature | Description |
|--|--|--|
| `std::io::stdin::read_line()` | `pub fn read_line(&self, buf: &mut String) -> Result<usize>` | Read a line of user input |
| `std::println!()` | `std::println` | Print to standard output, with a newline |
----------------------------------------

### Notes

`::` indicates an *associated function*, which is implemented on a type, not a particular instance of a type. Some languages call this a *static method*.

A crate is a collection of Rust source code files. 