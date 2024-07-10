use std::io;

pub fn learning_io() {
    let mut input = String::new();
    println!("Please enter you name: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Your Name is : {}", input);
}
