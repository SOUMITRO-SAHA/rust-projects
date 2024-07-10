### Pass by Value

When you pass a variable by value, you are passing a copy of the variable to the function. This means the function gets its own copy and changes to it do not affect the original variable.

**Example:**

```rust
fn print_and_modify_value(mut num: i32) {
    num += 1;
    println!("Inside function: {}", num);
}

fn main() {
    let number = 5;
    print_and_modify_value(number); // Pass by value
    println!("Outside function: {}", number);
}
```

**Explanation:**

- The `print_and_modify_value` function takes an integer `num` by value.
- Inside the function, `num` is modified, but this change does not affect the original `number` in `main`.
- Output:
  ```
  Inside function: 6
  Outside function: 5
  ```

### Pass by Reference

When you pass a variable by reference, you are passing a reference to the original variable. This allows the function to borrow the variable and potentially modify it if the reference is mutable.

**Example:**

```rust
fn print_and_modify_reference(num: &mut i32) {
    *num += 1;
    println!("Inside function: {}", num);
}

fn main() {
    let mut number = 5;
    print_and_modify_reference(&mut number); // Pass by reference
    println!("Outside function: {}", number);
}
```

**Explanation:**

- The `print_and_modify_reference` function takes a mutable reference `num` to an integer.
- Inside the function, `*num` is used to modify the original `number`.
- Output:
  ```
  Inside function: 6
  Outside function: 6
  ```

### Pass by Pointer

Rust does not have traditional pointers like C or C++. Instead, it uses references and smart pointers (e.g., `Box`, `Rc`, `Arc`). However, the concept of passing a raw pointer can be demonstrated with `*const T` and `*mut T` for unsafe code.

**Example:**

```rust
use std::ptr;

fn print_and_modify_pointer(num: *mut i32) {
    unsafe {
        if !num.is_null() {
            *num += 1;
            println!("Inside function: {}", *num);
        }
    }
}

fn main() {
    let mut number = 5;
    let num_ptr: *mut i32 = &mut number as *mut i32; // Create a mutable pointer
    print_and_modify_pointer(num_ptr); // Pass by pointer
    println!("Outside function: {}", number);
}
```

**Explanation:**

- The `print_and_modify_pointer` function takes a raw mutable pointer `num` to an integer.
- Inside the function, `unsafe` block is used to dereference and modify the pointer.
- This directly modifies the original `number`.
- Output:
  ```
  Inside function: 6
  Outside function: 6
  ```

### Ownership and Borrowing in Rust

Rust's ownership model plays a crucial role in understanding these concepts. Let’s apply this to the original example:

```rust
enum BloodType {
    AB,
    A,
    B,
    O,
}

fn print_blood_type(option: BloodType) {
    match option {
        BloodType::AB => println!("AB"),
        BloodType::A => println!("A"),
        BloodType::B => println!("B"),
        BloodType::O => println!("O"),
    }
}

fn main() {
    let blood_type = BloodType::AB;
    print_blood_type(blood_type); // First use, blood_type is moved
    print_blood_type(blood_type); // Error: blood_type has been moved
}
```

**Explanation:**

- `BloodType` is passed by value to `print_blood_type`. When `print_blood_type` is called the first time, `blood_type` is moved to the function.
- After the first call, `blood_type` is no longer valid in `main`, leading to a compile-time error on the second call.

**Fix: Use References:**

```rust
enum BloodType {
    AB,
    A,
    B,
    O,
}

fn print_blood_type(option: &BloodType) { // Take reference to BloodType
    match option {
        BloodType::AB => println!("AB"),
        BloodType::A => println!("A"),
        BloodType::B => println!("B"),
        BloodType::O => println!("O"),
    }
}

fn main() {
    let blood_type = BloodType::AB;
    print_blood_type(&blood_type); // Pass by reference
    print_blood_type(&blood_type); // Pass by reference again
}
```

**Explanation:**

- By passing a reference `&BloodType` to `print_blood_type`, we allow the function to borrow `blood_type` without taking ownership.
- This allows multiple uses of `blood_type` in `main`.

**Output:**

```
AB
AB
```

This demonstrates how Rust ensures memory safety through ownership and borrowing, and how you can use references to avoid moving ownership when not necessary.
