pub fn run_string() {
    // 1. String Literals
    // String literals are immutable and stored in the binary.
    let string_literal = "This is a string literal";
    println!("String Literal: {}", string_literal);

    // 2. String Objects
    // String objects are heap-allocated and mutable.
    let string_object_from_literal = String::from("This is a String object created from a literal");
    let string_object_from_to_string = "Soumitra Saha".to_string();

    println!(
        "String Object (from literal): {}",
        string_object_from_literal
    );
    println!(
        "String Object (using to_string): {}",
        string_object_from_to_string
    );

    // 3. Concatenation
    // Concatenating two String objects.
    let first_part = String::from("Hello");
    let second_part = String::from(", Rust!");
    let concatenated_string = first_part + &second_part; // first_part is moved and cannot be used again
    println!("Concatenated String: {}", concatenated_string);

    // 4. Appending to String
    // Using push_str and push to append to a String.
    let mut mutable_string = String::from("Hello");
    mutable_string.push_str(", Rustaceans!"); // Append a string slice
    mutable_string.push('!'); // Append a single character
    println!("Appended String: {}", mutable_string);

    // 5. Replacing Substrings
    // Using replace method to replace substrings within a String.
    let original_string = String::from("I love Rust programming!");
    let replaced_string = original_string.replace("love", "enjoy");
    println!("Original String: {}", original_string);
    println!("Replaced String: {}", replaced_string);

    // 6. Formatting Strings
    // Using format! macro to create formatted strings.
    let name = "Alice";
    let age = 30;
    let formatted_string = format!("Hello, {}! You are {} years old.", name, age);
    println!("Formatted String: {}", formatted_string);

    // 7. Iterating Over Characters
    // Using chars() method to iterate over characters in a String.
    let sample_string = String::from("Rust");
    for c in sample_string.chars() {
        println!("Character: {}", c);
    }

    // 8. Unicode Support
    // Demonstrating Unicode support in Rust strings.
    let unicode_string = String::from("🚀 Rust is awesome! 🦀");
    println!("Unicode String: {}", unicode_string);
    println!("Number of characters: {}", unicode_string.chars().count());

    // 9. Parsing Strings to Other Data Types
    // Using parse method to convert strings to other data types with error handling.
    let numeric_string = "42";
    match numeric_string.parse::<u32>() {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error parsing number: {}", e),
    }

    // 10. Trimming Whitespace
    // Using trim method to remove leading and trailing whitespace from a String.
    let string_with_whitespace = "  Rust is fun!  ";
    let trimmed_string = string_with_whitespace.trim();
    println!(
        "Original String with Whitespace: '{}'",
        string_with_whitespace
    );
    println!("Trimmed String: '{}'", trimmed_string);

    // 11. String Length
    // Using len method to get the length of a String.
    let example_string = String::from("Rustacean");
    println!("Length of '{}': {}", example_string, example_string.len());
}
