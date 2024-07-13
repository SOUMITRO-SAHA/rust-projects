// Concrete: A function that finds the largest number in a list of i32 values
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Concrete: A function that finds the largest number in a list of char values
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generics: A function that finds the largest item in a list of any type that can be compared
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    // Using the concrete functions
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result); // Outputs: The largest number is 100

    let result = largest_char(&char_list);
    println!("The largest char is {}", result); // Outputs: The largest char is y

    // Using the generic function
    let result = largest(&number_list);
    println!("The largest number is {}", result); // Outputs: The largest number is 100

    let result = largest(&char_list);
    println!("The largest char is {}", result); // Outputs: The largest char is y
}
