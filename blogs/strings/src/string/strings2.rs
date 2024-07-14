pub fn run_string_2() {
    // A reference to a string allocated in read-only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Original Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("\nWords in reverse order:");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    println!(
        "\nCharacters in the pangram (sorted and duplicates removed): {:?}",
        chars
    );

    // Create an empty and growable `String`
    let mut unique_chars_string = String::new();
    for c in chars {
        // Insert a char at the end of the string
        unique_chars_string.push(c);
        // Insert a string at the end of the string
        unique_chars_string.push_str(", ");
    }

    println!(
        "\nUnique characters concatenated with comma and space: {}",
        unique_chars_string
    );

    // The trimmed string is a slice to the original string, hence no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = unique_chars_string.trim_matches(chars_to_trim);
    println!("Used characters after trimming: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    println!("\nOriginal sentence: {}", alice);

    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dogs", "cats");
    println!("Modified sentence: {}", bob);
}
