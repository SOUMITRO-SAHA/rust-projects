use basic1::my_module;

fn main() {
    // Use a function from the library crate
    let greeting = basic1::say_hello();
    println!("{}", greeting);

    // Use a function from a module in the library crate
    let custom_greeting = my_module::greet("Alice");
    println!("{}", custom_greeting);
}
