// A function to find the largest element in a list using generics.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// A generic struct to represent a point with x and y coordinates of the same type.
struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

// Implementing methods for the Point<T> struct.
impl<T> Point<T> {
    // A method to get a reference to the x field.
    pub fn x(&self) -> &T {
        &self.x
    }
}

// A generic struct to represent a point with x and y coordinates of different types.
struct Point2<T, U> {
    pub(crate) x: T,
    pub(crate) y: U,
}

// Implementing methods for the Point2<T, U> struct.
impl<T, U> Point2<T, U> {
    // A method to mix up two points and create a new point with the x from self and the y from another point.
    pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// A function to test generics in Rust.
pub fn run_generic_tests() {
    // Testing the generic function largest
    println!("==========================");
    println!("Testing Generic Function: largest");

    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
    println!("==========================");

    // Testing the generic struct Point<T>
    println!("Testing Generic Struct: Point<T>");

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
    println!("==========================");

    // Testing the generic struct Point2<T, U>
    println!("Testing Generic Struct: Point2<T, U>");

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "Both Integer Point: ({}, {})",
        both_integer.x, both_integer.y
    );
    println!("Both Float Point: ({}, {})", both_float.x, both_float.y);
    println!(
        "Integer and Float Point: ({}, {})",
        integer_and_float.x, integer_and_float.y
    );
    println!("==========================");

    // Testing methods on generic struct Point<T>
    println!("Testing Methods on Generic Struct: Point<T>");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("==========================");

    // Testing the Option and Result enums
    println!("Testing Option and Result enums:");
    let some_value = Option::Some(42);
    let none_value: Option<i32> = Option::None;
    let ok_result: Result<i32, &str> = Result::Ok(42);
    let err_result: Result<i32, &str> = Result::Err("Error occurred");
    println!("Some value: {:?}", some_value);
    println!("None value: {:?}", none_value);
    println!("Ok result: {:?}", ok_result);
    println!("Err result: {:?}", err_result);
}
