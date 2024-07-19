# Option

- A type that may be one of two things
  - Some data of a specific type (`Some(T`))
  - Nothing (`None`)
- Used in scenarios where data may not be required or is unavailable
  - Unable to find something
  - Ran out of items in a list
  - Form field not filled out

## Definition

```rs
enum Option<T>{
    Some(T),
    None,
}
```

### Example

```rs
struct Customer {
    age: Option<i32>,
    email: String,
}

let mark = Customer {
    age: Some(24), email: "demo@example.com".to_owned(),
}

let becky = Customer {
    age: Some(24), email: "becky@example.com".to_owned(),
}

match becky.age {
    Some(age) => println!("customer is {:?} years old", age)
    None => println!("customer age not provided")
}
```
