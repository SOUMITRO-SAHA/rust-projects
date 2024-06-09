use std::fmt::Display;

/// A generic struct to represent a point with x and y coordinates of the same type.
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    /// Creates a new Point instance.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point.
    /// * `y` - The y-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A new instance of Point.
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Point<T> {
    /// Displays the values of the Point instance.
    fn display(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

/// Function to test methods with trait bounds.
pub fn run_trait_bounds_method_test() {
    println!("--- Running Trait Bounds Method Test ---");
    println!("This test demonstrates using trait bounds in method implementations.");

    let point = Point::new(3, 4);
    println!("\nBefore displaying Point:");
    println!("Point: x = {}, y = {}", point.x, point.y);

    println!("\nDisplaying Point using display method:");
    point.display();

    println!("--- End of Trait Bounds Method Test ---");
}
