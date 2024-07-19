#[derive(Debug)]
struct Address {
    address1: Option<String>,
    address2: Option<String>,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: f64,
    email: Option<String>,
    address: Address,
}

fn main() {
    let john = Person {
        id: 1,
        name: "John Doe".to_string(),
        age: 30.5,
        email: Some("john.doe@example.com".to_string()),
        address: Address {
            address1: Some("123 Main St".to_string()),
            address2: None,
            city: "New York".to_string(),
            state: "NY".to_string(),
            zip: "10001".to_string(),
        },
    };

    println!("John {:?}", john);
    println!("John Doe's address: {:?}", john.address);
}
