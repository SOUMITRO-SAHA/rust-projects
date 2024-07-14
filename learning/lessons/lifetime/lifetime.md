# Lifetime

[Read in details](https://practice.course.rs/lifetime/basic.html)

- Another kind of generic ensuring that references are valid as long as needed
- Every reference has a lifetime, which is the scope for which that reference is valid
- Most of the time implicit and inferred, don't need to worry
- Sometimes lifetime annotations are needed, if the compiler can't infer it
- Lifetime annotations is a concept which most other programming languages don't have

## Dangling References/ Dangling Pointers

```rs
fn main() {
    let r;                      // -----------+--- 'a
                                //            |
    {                           //--+--- 'b   |
        let x = 5;              //  |         |
        r = &x;                 //  |         |
    }                           //--+         |
                                //            |
    println!("r: {}", r);       //------------+
}

```

- Main aim of lifetime is to prevent dangling references (also called dangling pointers).
- Outer scope declares variable r with no initial value. Inner scope declares variable x with initial value of 5. Then a reference of x is assigned to r.
- x then goes out of scope and r refers to has gone out of scope.

## Borrow Checker

- Borrow checker **compares scopes** to determine whether **all borrows are valid**
- Key part of Rust's **ownership** system
- Tracks lifetime of **references** and ensures that they don't violate the **ownership rules**
- Rules ensure that a value is **not accessed** after it has been **moved** or **freed** from memory
- **Important**: A reference to a value **must never outlive the value itself!**

## Lifetime annotating

The **borrow checker uses explicit lifetime annotations** to determine how long a reference should be valid.

But for us users, in most cases, there is no need to annotate the lifetime, because there are several elision rules, before learning these rules, we need to know how to annotate lifetime manually.

### Function

Ignoring elision rules, lifetime in function signatures have a few constraints:

- Any reference must have an annotated lifetime
- Any reference being returned must have the same lifetime as one of the inputs ro be static

## Three Rules of Lifetime Elision

- The compiler uses **three rules** to figure out lifetimes of references that aren't **explicit** annotation
  - Compiler **assigns a lifetime** parameter to each parameter that's reference
  - If there is **exactly one** input lifetime parameter that lifetime is assigned to **all output** lifetime parameters
  - If there are multiple lifetime parameters but one of them is `&self` or `&mut` self the **lifetime of self is assigned to all output lifetime parameters**

```rs
fn first_word(s: &str) -> &str {};
```

to

```rs
fn first_word<'a>(s: &'a str) -> &str {};
```

to

```rs
fn first_word<'a>(s: &'a str) -> &'a str {};
```

- Compiler applies **first rule**: Each parameter gets its own lifetime
- **Second rule** applies because there is exactly **one input** lifetime, so the lifetime of the one input parameter gets assigned to the output lifetime
- In this case, the compiler could **infer** the lifetime and we don't have to specify them **manually**.

### Example Lifetime Elision

#### Example 1

```rs
fn longest(x: &str, y: &str) -> &str {}
```

- First rule: Each parameter gets its own lifetime

```rs
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

- Here second rule doesn't apply because there is more than one input lifetime. Also, third rule doesn't apply because this function is not a method
- We have to manually annotate the lifetime parameters.

#### Example 2

```rs
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        self.announcement = announcement;
        println!("Attention please: {}", announcement);
        self.announcement
    }
}
```

- Here the third rule applies, which states that if there is a reference to self `(&self)` then all references will have the same lifetime as `&self`

## Static Lifetime

- Refers to a lifetime that lasts for the **entire duration** of the program's **execution**
- Any reference or borrowed value with static lifetime can be safely used throughout the program
- Can be `coerced` to a shorter lifetime if needed

```rs
let s: &str => "Hello, world";

// or,
let s: &'static str = "Hello, world";
```

- String literals have a static lifetime because they are hardcoded into the executable meaning they are valid throughout the entire duration of the program's execution
