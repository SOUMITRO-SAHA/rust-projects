# Generics

- Placeholders for concrete types
- Enables writing more reusable and flexible code
- Avoids having duplicate code for different types
- Zero cost abstraction, Rust compiler will at compile time fill out generics with concrete types

## Const Generics

- Type parameter that represents a compile-time constant value.
- Allows to write generic code that operates on values that are known at compile time
- Used for array sizes, bit widths and other constants

```rs
struct A; // Concrete type 'A'
struct S(A); // Concrete type 'S'
struct SGen<T>(T); // Generic type 'SGen'
```

- A function call with explicitly specified type parameters looks like `fun::<A, B, ...>()`

```rs
use std;

fn sum<T: std::ops::Add<Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.3, 3.2), 4.5);
    println!("Success");
}
```

- Generic Struct

```rs
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

fn main() {
    let integer_point: Point<i32> = Point{x: 1, y:5};
    let decimal_point: Point<f64> = Point{x:1.1, y: 4.5};

    println!("{:?}", integer_point);
    println!("{:?}", decimal_point);
}
```

- **Const Generics** `<T, const N: usize>` is part of the struct type, it means Array<i32, 3> and Array<i32, 4> are different types

```rs
// Define a struct `Array` that uses const generics
struct Array<T, const N: usize> {
    elements: [T; N], // An array with `N` elements of type `T`
}

impl<T, const N: usize> Array<T, N> {
    // Method to create a new `Array` instance
    fn new(elements: [T; N]) -> Self {
        Array { elements }
    }

    // Method to get the length of the array
    fn len(&self) -> usize {
        N
    }

    // Method to get an element at a specific index
    fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.elements[index])
        } else {
            None
        }
    }
}

fn main() {
    // Create instances of the `Array` struct with different sizes
    let array_3 = Array::new([1, 2, 3]);
    let array_4 = Array::new([1, 2, 3, 4]);

    // Print the length of the arrays
    println!("Length of array_3: {}", array_3.len()); // Outputs: Length of array_3: 3
    println!("Length of array_4: {}", array_4.len()); // Outputs: Length of array_4: 4

    // Access elements in the arrays
    println!("Element at index 1 in array_3: {:?}", array_3.get(1)); // Outputs: Element at index 1 in array_3: Some(2)
    println!("Element at index 3 in array_4: {:?}", array_4.get(3)); // Outputs: Element at index 3 in array_4: Some(4)
    
    // Attempt to access an out-of-bounds element
    println!("Element at index 4 in array_3: {:?}", array_3.get(4)); // Outputs: Element at index 4 in array_3: None
}

```
