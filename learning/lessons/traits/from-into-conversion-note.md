# From/Into Conversion

- `Form` and `Into` traits are used for **type conversions** between different types **without requiring explicit casts**
- Part of standard library
- Can be implemented for **custom types**
- **Implementing** From for a type will give us Into implementation for the given type **for free!!!**

```rs
#[derive(Debug)]

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        Number {
            value: n,
        }
    }
}

fn main() {
    let num: Number = Number::from(30);
    
    assert_eq!(num.value, 30);
    
    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);
    
    println!("Success");
}
```

- Here we have created a struct Number with one field value. We want to be able to convert an i32 value directly to a Number type value in the value field.

- We do this by implementing the From trait for our custom type Number and provide the customize from method

## TryFrom/TryInto

- Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.

Unlike From/Into, TryFrom and TryInto are used fro fallible conversions and return a Result instance of a plain value.
