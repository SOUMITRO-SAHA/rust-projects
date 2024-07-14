# Basic-1

## `lib.rs`

- **Purpose**: `lib.rs` is used to define a library crate in Rust. A library crate is a collection of Rust code that provides functionality to other projects.
- **Usage**: Library crates are intended to be reused by other projects and do not have an entry point (i.e., a `main` function).
- **Structure**: The code in `lib.rs` typically consists of modules, functions, types, and other items that you want to expose as part of the library’s API.

### Example of `lib.rs`

```rust
// lib.rs

// Declare a module
pub mod my_module {
    // Define a public function
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    // Define a private function
    fn private_function() {
        println!("This is a private function.");
    }
}

// Define a function in the root of the library
pub fn say_hello() -> String {
    String::from("Hello from the library!")
}
```

## `main.rs`

- **Purpose**: `main.rs` is used to define a binary crate in Rust. A binary crate is an executable program.
- **Usage**: Binary crates are standalone programs that have an entry point defined by the `main` function.
- **Structure**: The code in `main.rs` typically includes the `main` function and may use functions, types, and modules from the project's own library crate or other external crates.

### Example of `main.rs`

```rust
// main.rs

// Import the library crate
extern crate my_lib;

fn main() {
    // Use a function from the library crate
    let greeting = my_lib::say_hello();
    println!("{}", greeting);

    // Use a function from a module in the library crate
    let custom_greeting = my_lib::my_module::greet("Alice");
    println!("{}", custom_greeting);
}
```

### Differences and Usage

1. **Project Types**:
   - **Library Crate**: If your project is a library crate, you will typically have a `lib.rs` file but no `main.rs` file.
   - **Binary Crate**: If your project is a binary crate (an executable program), you will have a `main.rs` file.
   - **Hybrid Project**: It’s common to have a project with both `lib.rs` and `main.rs`, where `lib.rs` defines reusable functionality, and `main.rs` contains the entry point for an executable that uses the library.

2. **Location**:
   - **lib.rs**: Located in the `src` directory of a library crate or a hybrid project.
   - **main.rs**: Located in the `src` directory of a binary crate or a hybrid project.

3. **Cargo Configuration**:
   - **Library Crate**: Your `Cargo.toml` file will include `[lib]` section.
   - **Binary Crate**: Your `Cargo.toml` file will include `[bin]` section, though it can be omitted if you only have one binary crate, as Cargo defaults to `main.rs`.

### Example Project Structure

Here’s an example project structure for a hybrid project that includes both `lib.rs` and `main.rs`:

```bash
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

### Cargo.toml Example

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

This structure allows you to define reusable functionality in `lib.rs` and use it in the executable defined by `main.rs`.