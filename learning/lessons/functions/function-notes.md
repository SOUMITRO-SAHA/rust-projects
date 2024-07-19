# Functions

- A way to encapsulate program functionality
- Block of reusable code that performs a specific tasks
- Can take arguments, processes those inputs and returns a result

```rs
fn main() {
    let result = sum(1, 2);
    println!("result is {}", result);
}

fn sum(a: i32, b: i32) {
    a + b;
}
```

## Diverging functions

- Never return to the caller
- E.g. panic, looping forever, quitting the program

```rs
fn never_return() -> ! {
    panic!();
}
```
