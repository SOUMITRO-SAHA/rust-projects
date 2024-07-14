# Methods

- Function that is **associated** with a **particular type** or **struct**
- Takes **parameters** and **returns a value**, but defined as a **member** of a struct or enum
- Called using **dot notation** (like accessing members of a struct)
- Implemented through an `impl` block

```rs
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r:Rectangle = Rectangle{width: 10, height: 10}
    println!("Area = {}", r.area());
}
```

## Associated Function

- Function that is associated with a struct or an enum, but does not take an instance as its first parameter
- Called using the name of the type, not an instance of it
- Often used as constructor for a struct or enum

## Important Points

1. Methods are similar to functions: Declare with `fn`, have parameters and a return value. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

2. `Self` will take the ownership of current struct instance, However, `&self` will only borrow a reference from the instance

```rs
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("The current state is {}", self.color);
    }

    pub fn change_state(&mut self){
        self.color = "green".to_string();
    }
}

fn main() {
    println!("Success");
}
```

### Associate Function

- All functions defined within an `impl` block are called associated functions because they're associated with the type named after the `impl`. We can define associated functions that don't have `self` as their first parameter (and thus are not methods) because they don't need an instance of the type to work with.

- Each struct is allowed to have multiple impl blocks.
- We can also implement methods for enums.

```rs
#[derive(Debug)]
enum TrafficLightColor {
    Red, 
    Yellow, 
    Green, 
}

impl TrafficLightColor {
    pub fn color(&self) -> &str{
        match self {
            self::Red => "Red",
            self::Yellow => "Yellow",
            self::Green => "Green",
            _ => "Unknown"
        }

        /* or 
        match self {
            TrafficLightColor::Red => "Red",
            TrafficLightColor::Yellow => "Yellow",
            TrafficLightColor::Green => "Green",
            _ => "Unknown"
        }
        */
    };
}

fn main() {
    let color:TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(color.color(), "Yellow");

    println!("{:?}", color);
    println!("Success");
}
````
