# Enum

- Data that can be one of multiple different possibilities
  - Each possibility is called a "variant".
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

```rs
// Define an enum with explicit discriminators
enum Status {
    Active = 1,
    Inactive = 0,
    Pending = 2,
}

fn main() {
    // Create instances of the enum variants
    let active_status = Status::Active;
    let inactive_status = Status::Inactive;
    let pending_status = Status::Pending;

    // Print the values of the enum variants
    println!("Active status: {}", active_status as i32); // Outputs: Active status: 1
    println!("Inactive status: {}", inactive_status as i32); // Outputs: Inactive status: 0
    println!("Pending status: {}", pending_status as i32); // Outputs: Pending status: 2

    // Use a match statement to handle different variants
    match active_status {
        Status::Active => println!("The status is active."),
        Status::Inactive => println!("The status is inactive."),
        Status::Pending => println!("The status is pending."),
    }
}
```

- Each enum variant can hold its own data
- We can get the data which an enum variant is holding by pattern match

```rs
// Define an enum where each variant holds different types of data
enum Message {
    Quit,                       // No data associated with this variant
    Move { x: i32, y: i32 },    // Named fields
    Write(String),              // Tuple struct
    ChangeColor(i32, i32, i32), // Tuple struct with multiple values
}

fn main() {
    // Create instances of the enum variants
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("Hello, Rust!"));
    let color_message = Message::ChangeColor(255, 0, 0);

    // Function to process the messages | Pattern Matching
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quit message received."),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB: ({}, {}, {})", r, g, b),
        }
    }

    // Process each message
    process_message(quit_message);
    process_message(move_message);
    process_message(write_message);
    process_message(color_message);
}
```

- Since there is not null in Rust, we have to use enum `Option<T>` to deal with the cases when the value is absent.

```rs
// Define an enum Option with variants Some and None
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // Create instances of Option
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // Function to add one to an Option<i32>
    fn plus_one(option: Option<i32>) -> Option<i32> {
        match option {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    // Use the plus_one function
    let six = plus_one(some_number);
    let none = plus_one(no_number);

    // Print the results
    println!("six: {:?}", six); // Output: six: Some(6)
    println!("none: {:?}", none); // Output: none: None
}
```

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
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty list
    let mut list = List::new();

    // Prepend elements to the list
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Print the length of the list
    println!("Length: {}", list.len());

    // Print the list
    println!("{}", list.stringify());
}
```
