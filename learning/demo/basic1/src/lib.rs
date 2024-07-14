pub mod my_module {
    // Define a public function
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    // Define a private function
    fn private_function() {
        println!("This is a private function.");
    }
}

// Define a function in the root of the library
pub fn say_hello() -> String {
    String::from("Hello from the library!")
}
