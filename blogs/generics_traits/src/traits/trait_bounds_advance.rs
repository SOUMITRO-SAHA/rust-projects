use std::fmt::Display;

/// Prints a pair of values that implement the Display trait.
///
/// # Arguments
///
/// * `pair` - A tuple containing two elements that implement the Display trait.
fn print_display_pair<T, U>(pair: (T, U))
where
    T: Display,
    U: Display,
{
    println!("First: {}, Second: {}", pair.0, pair.1);
}

/// A generic struct that holds a pair of values, both of which must implement the Display trait.
struct DisplayPair<T, U>
where
    T: Display,
    U: Display,
{
    x: T,
    y: U,
}

impl<T, U> DisplayPair<T, U>
where
    T: Display,
    U: Display,
{
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
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    /// Displays the values of the DisplayPair instance.
    fn display(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

/// Function to demonstrate advanced trait bounds concepts.
pub fn run_advanced_trait_bounds_tests() {
    println!("--- Running Advanced Trait Bounds Tests ---");
    println!("This test demonstrates how trait bounds are used in structs and functions.");

    // Create and display a DisplayPair instance.
    let display_pair = DisplayPair::new(1, "hello");
    println!("\nBefore displaying DisplayPair:");
    println!(
        "DisplayPair: x = {}, y = {}",
        display_pair.x, display_pair.y
    );
    println!("Displaying DisplayPair:");
    display_pair.display();

    // Create a tuple and print its values using the print_display_pair function.
    let pair_tuple = (1, "hello");
    println!("\nBefore printing tuple:");
    println!("Tuple: First = {}, Second = {}", pair_tuple.0, pair_tuple.1);
    println!("Printing tuple using print_display_pair:");
    print_display_pair(pair_tuple);

    println!("--- End of Advanced Trait Bounds Tests ---");
}
