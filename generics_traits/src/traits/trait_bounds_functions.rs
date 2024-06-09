use std::fmt::Debug;
use std::fmt::Display;

/// Prints an item that implements the Display trait.
///
/// # Arguments
///
/// * `item` - The item to be printed.
fn print_display_item<T: Display>(item: T) {
    println!("{}", item);
}

/// Prints an item that implements both Display and Debug traits.
///
/// # Arguments
///
/// * `item` - The item to be printed.
fn print_debug_and_display_item<T: Display + Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

/// Function to test basic trait bounds with Display trait.
pub fn run_basic_trait_bounds_test() {
    println!("--- Running Basic Trait Bounds Test ---");
    println!("This test demonstrates using the Display trait with a single bound.");

    let number = 42;
    let text = "Hello, world!";

    println!("\nBefore printing items with Display trait:");
    println!("Number: {}", number);
    println!("Text: {}", text);

    println!("\nPrinting items with Display trait using print_display_item:");
    print_display_item(number);
    print_display_item(text);

    println!("--- End of Basic Trait Bounds Test ---");
}

/// Function to test multiple trait bounds with Display and Debug traits.
pub fn run_multiple_trait_bounds_test() {
    println!("--- Running Multiple Trait Bounds Test ---");
    println!("This test demonstrates using both Display and Debug traits with multiple bounds.");

    let number = 42;
    let text = "Hello, world!";

    println!("\nBefore printing items with Display and Debug traits:");
    println!("Number: Display = {}, Debug = {:?}", number, number);
    println!("Text: Display = {}, Debug = {:?}", text, text);

    println!("\nPrinting items with Display and Debug traits using print_debug_and_display_item:");
    print_debug_and_display_item(number);
    print_debug_and_display_item(text);

    println!("--- End of Multiple Trait Bounds Test ---");
}
