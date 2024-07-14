use std::fmt::Display;

/// A generic struct that holds a pair of values, both of which must implement the Display trait.
struct DisplayPair<T: Display> {
    x: T,
    y: T,
}

impl<T: Display> DisplayPair<T> {
    /// Creates a new DisplayPair instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The first value of the pair.
    /// * `y` - The second value of the pair.
    ///
    /// # Returns
    ///
    /// A new instance of DisplayPair.
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Displays the values of the DisplayPair instance.
    fn display(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

/// Function to test struct with trait bounds.
pub fn run_trait_bounds_struct_test() {
    println!("--- Running Trait Bounds Struct Test ---");
    println!("This test demonstrates using trait bounds in struct definitions.");

    let pair = DisplayPair::new(1, 2);
    println!("\nBefore displaying DisplayPair:");
    println!("DisplayPair: x = {}, y = {}", pair.x, pair.y);

    println!("\nDisplaying DisplayPair using display method:");
    pair.display();

    println!("--- End of Trait Bounds Struct Test ---");
}
