enum BloodType {
    AB,
    A,
    B,
    O,
}

fn print_blood_type(option: &BloodType) {
    match option {
        BloodType::AB => println!("AB"),
        BloodType::A => println!("A"),
        BloodType::B => println!("B"),
        BloodType::O => println!("O"),
    }
}

fn main() {
    let blood_type = BloodType::AB;
    print_blood_type(&blood_type); // Passing a reference
    print_blood_type(&blood_type);
}
