// Define an enumeration called `BloodType` with four variants: `AB`, `A`, `B`, and `O`.
enum BloodType {
    AB,
    A,
    B,
    O,
}

// Define a function `print_blood_type` that takes an argument `option` of type `BloodType`.
// This function matches the `option` against each variant of the `BloodType` enum and prints the corresponding blood type.
fn print_blood_type(option: BloodType) {
    match option {
        BloodType::AB => println!("AB"),
        BloodType::A => println!("A"),
        BloodType::B => println!("B"),
        BloodType::O => println!("O"),
    }
}

// The main function is the entry point of the program.
fn main() {
    // Declare a variable `blood_type` and assign it the value `BloodType::AB`.
    let blood_type = BloodType::AB;

    // Call the function `print_blood_type`, passing `blood_type` as an argument.
    // Ownership of the value `blood_type` is moved into the function `print_blood_type`.
    print_blood_type(blood_type);

    // Attempt to call `print_blood_type` again, passing `blood_type` as an argument.
    // This causes an error because `blood_type` was moved in the previous function call,
    // and therefore, it is no longer valid here.
    print_blood_type(blood_type);
}
