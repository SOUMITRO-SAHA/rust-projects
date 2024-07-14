# Miscellaneous

## Pattern Match

- Powerful construct that allows to **compare a value** against a set of **patterns**, then **execute** different code based on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, etc..
- In match, **all possible cases** must be handled, enforced by the compiler

## if let

In a match statement, every case has to be handled. This sometimes leads to annoying boilerplate code that is not necessary. Instead we can use if let to unwrap a value of an Option type.

```rs
fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max);
        _ => (),
    }
}
```

Using if let

```rs
fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

- ⭐⭐⭐ The `@` operator lets use create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p:Point = Point{x: 0, y:0};

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0..=5, y: y @ (10 | 20 | 30)} => println!("On the y axis at {}",y);
        Point {x, y} => println!("On neither axis! x={}, y={}", x, y);
    }
}

```

- ⭐⭐ A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.

```rs
fn main() {
    let num:Option<i32> = Some(4);
    let split = 5;
    match sum {
        Some(x) if x < split => assert_eq!(x < split),
        Some(x) => assert_eq!(x >= split),
        None => (),
    }

    println!("Success!");
}
```

- ⭐⭐ Ignoring remaining parts of the value with `..`

```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 1024, 2048, 4096);

    match numbers {
        (first,..,last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 4096);
        }
    }
}
```

- ⭐⭐ Using pattern `&mut` V to match a mutable reference needs you to be very careful, due to V being a value after matching

```rs
fn main() {
    let mut v:String = String::from("Hello,");

    let r: &mut String = &mut v;

    match r {
        value => value.push_str(" World!!!");
    }
}

```

## `panic!`

- Simplest form of **error handling** is to use the `panic!` macro
- `panic!`  will print out an error message, unwind the stack and finally exit the program
- In multi-threaded programs it will exit the thread in which `panic!` occurs, not the whole program

## Result

- Result is an enum type that represents the outcome of an operation that could potentially fail
- Two possible variables:
  - Ok(T): A value T was found
  - Err(e): An error was found with a value e
- The expected outcome is Ok, the unexpected outcome is Err
- Since Result is an enum, the possible variants can be matched using a match pattern

## unwrap

- The `unwrap()` method takes as input a value of type Result and takes out the value which is wrapped inside Ok(T) in case of success or panic in case of error

## ?

- The ? operator is a shorthand way to propagate errors or unwrap Ok() results
- Basically the same as unwrap() but instead of panic returns an error
- Replaces an entire match statement
- Can be used in the main() function

## Type Alias

- way of giving a new name to an existing type

```rs
type U64 = u64;

fn main(){
    let num:U64 = 43;
}
```

- Don't confuse with associate type in trait

## map & and_then

- `map` & `and_then` are two common combinators for `Result<T, E>` (also for `Option<T>`)

```rs
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    // or n_str.parse::<i32>().and_then(|n| Ok(n + 2))
    n_str.parse::<i32>().map(|n| n + 2)
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success");
}

```

## Cargo

- Official package manager and building tool
- Helps automate tasks such as creating new projects, building, running, testing code and managing dependencies
- Crate is a compilation unit or Rust source code
- `crate.io` repository for Rust packages

## Crate

- Binary
  - Compiled into an executable binary
  - Basically a "program"
- Library
  - Compiled into a library
  - Reusable code that can be shared across multiple projects
- Crate root
  - Source file that is the root module of the crate
  - In binary: src/main.rs
  - In Libraries: src/lib.rs

## Modules

- Way of organizing code by grouping related items together
- Can be imported using namespaces avoiding naming collisions
- Also controls privacy of its items like functions, structs, enums, etc.
- When compiling the compiler starts from the crate root, then checks if modules are declared and looks for submodules
- Submodules could be directly written inline within curly braces, in a file which has the module name ending in `.rs` or in the directory which has the name of the module and a mod.rs file inside.it.

