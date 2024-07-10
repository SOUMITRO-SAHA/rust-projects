mod array;
mod loops;
mod topics;
mod user_input;
mod vector;

use array::learning_array;

fn main() {
    println!("Welcome to Rust Learning Project!");

    // Call functions from different topics
    topics::hello_world::run();

    // Array
    println!("===================== [Topics] =====================");
    println!("=> Array");
    learning_array();
    println!("=> Vector");
    vector::learning_vector();
    println!("=> Loops");
    loops::learning_loop();

    println!("=> User Input");
    user_input::learning_io();
}
