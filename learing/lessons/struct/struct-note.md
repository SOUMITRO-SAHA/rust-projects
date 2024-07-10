# Struct

- Compound type allowing to group together values of different types into a named data structure
- Similar to tuple, but each value has a **name** so values can be **accessed** through this name
- Have to be **instanciated** with data, think of it like the struct is the **template for the instances** you create from it.

```rs
struct Person{
    name: string,
    age: i32,
    height: f64,
}

fn main() {
    let p1 = Person{
        name: String::from("Soumitra Saha");
        age: 24;
        height: 6;
    } // Instance

    // Accessing and Mutating
    let mut p2 = Person {
        name: String::from("Rohan"),
        age: 24;
        height: 6.3;
    }

    // We can access and mutate single fields of struct
    p2.height = 6.1;
}
```

- Function returning Structs

```rs
fn person_builder(name: String, age: i32, height: f64){
    // Functions can instantiate and return struct
    Person {
        name, age, height
    }
}
```

- Struct Update Syntax

```rs
struct User{
    active: boolean,
    username: String,
    email: String,
    sign_in_count: i32,
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("Soumitra Saha"),
        email: String::from("demo@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: String::from("Different Name"),
        email: String::from("different_email@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User{
        email: String::from("test@gmail.com");
        ..user1
    };
}
```

## Tuple Structs

- Like normal structs but using tuple-like syntax for defining their fields
- Basically a named tuple
- Instantiated by parenthesis instead of curly braces
- Accessed through point notion

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32);

fn main(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0); // 2d
}
```

## Unit-Like Structs

- Structs without any fields
- Used when working with traits
- Doesn't store any data

### Important Points

- We must specify concrete values for each of the fields in struct.
- Unit struct don't have any fields. It can be used when you need to implement trait on some type but don't have any data that you want to store in the type itself.
- Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields. It's useful when you want to give the whole tuple a name, but don't care about the field's names.
- You can make a whole struct mutable when instantiating it, but Rust doesn't allow us to make only certain fields as mutable.
- We can use `#[derive(Debug)]` to make a struct printable.

### Partial Move

Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will e moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.

```rs
#[derive(Debug)]
struct File{
    name: String,
    data: String,
}

fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust by example".to_string(),
    };

    let _name: String = f.name; // Ownership transferred here

    // We can not use f.name any for, or it will panic

    println!("{}, {}, {:?}", f.name, f.data, f);
    
}
```
