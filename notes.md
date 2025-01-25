# The Rust Programming Language
(No Starch Press)

## Chapter 01 - Getting Started

----------------------------------------
### Helpful commands

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

### Conventions

- Formatting can be standardized using `rustfmt`
- `fn() {` function name, then space, then brace
- Indents are four spaces, not a tab
----------------------------------------

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

