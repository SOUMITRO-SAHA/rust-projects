# Enum

- Way of defining a type with only one of a possible set of values
- We can only access one variant of an enum at a time
- Can hold additional information using tuples
- Especially useful when using in match statements

```rs
enum IPAddress {
    v4(String), // These are variants not fields
    v6(String),
}

// IP addresses can either be v4 or v6
let home = IPAddress::v4(String::from("127.0.0.1"))
let loopback = IPAddress::v6(String::from("::1"));
```

- Enums can be created with explicit discriminator
- Each enum variant can hold its own data
- We can get the data which an enum variant is holding by pattern match
- Since there is not null in Rust, we have to use enum `Option<T>` to deal with the cases when the value is absent.

## The Option Enum

- Option is an enum that represents a value that **may or may not be present**
- Known in other languages as `null`, referring to the **absence of a value**
- Used to handle cases where a function or method **might fail** to return a value

```rs
enum Option<T> {
    None,
    Some(T),
}

fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(option: Option<i32>): Option<i32>{
    match option{
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### Implementing a Linked List using enum

```rs
use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil
}

// TODO:  
impl List {
    fn new() -> List {
        // `Nil` has type 'List'
        Nil
    }
}
```
