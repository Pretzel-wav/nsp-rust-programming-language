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
- `$ cargo doc --open` will open the docs **provided by all of your dependencies**. This is fussy; I had to run `$ cargo doc -p rand --open` to download the rand docs first, but afterward they were included in `$ cargo doc --open`.

#### Crates

- `std` - Standard library. Contains many basic features.
- `std::io` - i/o crate in the standard library. Contains things like `stdin()` for reading user input
- `std::cmp` -  Comparison crate in the standard library. Contains `Ordering` type, which is an enum of `[Less, Greater, Equal]`; the three outcomes possible when comparing two values.
- `rand` - Random generation library

#### Functions

| Name | Signature | Description |
|--|--|--|
| `std::io::stdin::read_line` | `pub fn read_line(&self, buf: &mut String) -> Result<usize>` | Read a line of user input |
| `std::println!` | `std::println` | Print to standard output, with a newline |
| `rand::thread_rng` | `pub fn thread_rng() -> ThreadRng` | Create a fast, pre-initialized random value (deprecated since 0.9.0; use `rand::rng()` instead) |
----------------------------------------

### Notes

`::` indicates an *associated function*, which is implemented on a type, not a particular instance of a type. Some languages call this a *static method*.

## Chapter 03 - Common Programming Concepts

### Summary

----------------------------------------

#### Conventions

- Constants are named `IN_ALL_CAPS_WITH_UNDERSCORES`
- Don't write code that overflows intentionally; instead, use the `Wrapping` type in the standard library.
- Indicate planned unused variables with an underscore: `let (_x, y, _z) = tup; // _x and _z are not used later in the code`
- Functions are named in `snake_case()`
- Almost always use `for` loops instead of `loop` or `while`. `for number in (1..4) {}`

#### Language Features

- Underscores can be placed in numeric literals to improve readability. `100_000` == `100000`
- **Shadowing** allows you to declare a new variable with the same name and value as a previous value, by using the `let` keyword with the same variable name
- Overflow wrapping is checked and panicks in debug mode, but values simply wrap when compiled in release mode.
- Rust has four scalar data types: integers, floating-point numbers, booleans, and characters
- Character literals are specified with `'`, string literals apre specified with `"`
- Char type is 4 bytes in size and represents a Unicode Scalar Value
- Rust has two compound types: Tuples and Arrays `(tuples)`, `[arrays]`
    - Tuples have a fixed length, cannot change in size, and can combine various types `let tup: (i32, f64, u8) = (500, 6.4, 1);`
        - Assign variables from a tuple by *destructuring* `let (x, y, z) = tup;`
        - Access variables in a tuple with `.` indexing `let x = tup.0;`
    - Arrays have a fixed length, cannot change in size, and only contain a single type `let a: [i32; 5] = [1, 2, 3, 4, 5];`
        - You can declare an array with a repeated value using `let a = [3; 200]; // array of length 200, every value is 3`
        - Access array elements with `[]` indexing `let x = a[0];`
    - For collections that can change in size, use Vectors
- Rust doesn't care where you define functions, only that they're defined somewhere
- Parameter types must be declared in the function signature
- Rust is an *expression-based* language. Statements and Expressions are different.
    - Statements perform an action and do not return a value `let y = 6;`
    - Expressions evaluate to a resulting value `{ x + 1 } `
- The return value of a function is the result of the final expression in that function
- Conditionals must be booleans, meaning `if number {}` is not a valid construct

##### Numeric types
| Length | Signed int | Unsigned int | Float |
|--|--|--|--|
| 8-bit | `i8` | `u8` | N/A |
| 16-bit | `i16` | `u16` | N/A |
| 32-bit | `i32` | `u32` | `f32` |
| 64-bit | `i64` | `u64` | `f64` |
| 128-bit | `i128` | `u128` | N/A |
| arch | `isize` | `usize` | N/A |

##### Integer literals
*Note: All literals except Byte can be suffixed by a type, e.g. `57u8`, and all literals can use an underscore as a visual separator, e.g. `1_000`*
| Number literal | Example |
|--|--|
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (u8 only) | `b'A'` |


----------------------------------------

### Notes

Differences between immutable variables and constants
- Declare constants with `const`
- You cannot use `mut` with constants; they're not just immutable by default, they're *always* immutable
- The data type of a constant must be annotated; they're never implicitly assigned
- Constants can be declared in any scope, including the global scope
- Constants may only be set to a constant expression, i.e. not to the result of a function call or any other value computed at runtime
- Constants are valid for the entire time a program runs, within the scope they were declared in

Differences between mutable variables and shadowing variables
- A compile-time error will occur if you try to assign an immutable variable without the `let` keyword
- The shadowed variable will be immutable after assignment
- Shadowing creates a new variable, just with the same name. So you're adding another value to memory.
- Shadowing allows you to use different types for the variables after transformation.
    - For example, if you are creating a variable for the number of spaces to add to a string, you could write something like:
    ```rust
    let spaces = "   ";         // returns &str
    let spaces = spaces.len();  // returns usize
    ```
    - This is allowed because you have a &str-typed variable in the first `spaces`, and a usize-typed variable in the second `spaces`. Using `mut` wouldn't work in this instance:
    ```rust
    let mut spaces = "   ";
    spaces = spaces.len(); // raises a data type error! spaces is a &str, but .len() returns a usize
    ```
    - This allows you to avoid creating new variable names, like `spaces_str` and `spaces_int`

Differences between statements and expressions
- Statements perform an action and do not return a value
```rust
fn main() {
    let x = (let y = 6); // "expected expression, found statement (`let`)" error. let does not return a value!
}
```
- Expressions perform an action and do return a value
```rust
fn main() {
    let x = 5; // `let x = 5` is a statement binding 5 to x. `5` is an expression evaluating to 5, which gets bound to x in the statement

    let y = {      // curly braces indicate an expression
        let x = 3; // same as `let x = 5`
        x + 1      // an expression evaluating to x + 1. No semicolon because it's an expression!
    };             // the end of the curly brace block evaluates the expression, which gets bound to y in the statement

    println!("The value of y is {}", y); // the macro is an expression evaluated with its input, which is then executed in the statement (note the semicolon)
}
```

## Chapter 04 - Understanding Onwership

### Summary

----------------------------------------

#### Language Features

- `"String Literals"` are immutable `str`
- When a variable goes out of scope, Rust calls a special function `drop` to return the memory. Rust calls `drop` automatically at the closing bracket.
- Rust will never automatically create "deep" copies of your data. Therefore, any automatic copying can be assumed to be inexpensive.
- Rust has a special annotation called the `Copy` trait that is applied to types like integers that are stored on the stack. If a type has the `Copy` trait, the older variable is still usable after assignment.
- Rust won't let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the `Drop` trait.
- Passing a variable to a function will follow the same copy rules as variable assignment does.
- `&` references create pointers to the reference on the stack, without taking ownership of them.
- Having references as function parameters is called **borrowing**.
- Only one mutable reference at a time can be made on a particular piece of data in a particular scope. This prevents data races.
- String slices `&str[start..end]` are inclusive at lower bound, exclusive at upper bound
----------------------------------------

### Notes

Ownership Rules
- Each value has a variable in Rust that's called its *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

Calling `String::from` (and similar functions) requests memory from the operating system at runtime

If two variables are pointing to the same data on the heap, when `drop` is called by Rust, it will try to free the same data twice. This causes a *double free* error, and is why there can only be a single owner!

```rust
// Copying the reference without copying the heap data is known as a
// "shallow copy" in other languages. But since Rust invalidates the
// old variable, it's known as a "move" in Rust.
let s1 = String::from("hello"); // allocate data on heap, create reference on stack
let s2 = s1; // copy reference on stack, invalidate s1 variable
```

Types with `Copy` trait:
- All integers
- All floats
- `bool`
- `char`
- Tuples and Arrays, if they only contain `Copy` types

A data race happens when these three things occur:
- Two or more pointers access the same data at the same time
- At least one of the pointers is being used to write to the data
- There's no mechanism being used to synchronizze access to the data.

## Chapter 05 - Using Structs to Structure Related Data

### Summary

----------------------------------------

#### Language Features

- Use dot notation to access values from structs `user1.email`
- You can define an entire instance of a struct as mutable, but not parts of it. `user1` can be mutable, but `user1.email` cannot be mutable without `user1.username` also being mutable.
- Using *field init shorthand* syntax allows you to avoid repeating tedious build statements
```rust
fn build_user(email: String, username: String) -> User {
    User {
        username, // not username = username
        email     // not email = email
    }
}
```
- Using *struct update syntax* allows you to initialize a struct instance with the values from another instance
```rust
let user3 = User {
        email: String::from("yetanother@example.com"),
        username: String::from("thirdusername"),
        ..user1 // "use the rest of the values from user1"
    };
```
- *Tuple structs* are structs without field names. They're useful for adding the extra semantics of a struct, but less verbose.
    - Tuple structs are different types, so a function that takes a `Color` argument will error if a `Point` is passed to it, despite both structs having a signature of `(i32, i32, i32)`.
```rust
struct Color(i32, i32, i32);
let black = Color(0,0,0);
```
- Structs with no fields are called *unit-like structs*, because they are similar to `()`, the unit type.
----------------------------------------

### Notes

