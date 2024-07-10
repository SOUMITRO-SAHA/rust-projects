use std::ops::{Range, RangeInclusive};

fn main(){
    /* For Loops */
    // Example-1
    let mut sum = 0;

    for i in 0..10{
        sum += i;
    }

    println!("The sum is {sum}");

    // Example-2 (with characters)
    for ch in 'a'..'e'{
        println!("char => {ch}");
    }

    // Example-3


   // Exercises
   exercise_1();
   exercise_2();
}

fn exercise_1(){
    assert_eq!((1..5), Range{start:1, end: 5}); // Here, end=5 is exclusive
    assert_eq!((1..=5),  RangeInclusive::new(1, 5)); // Here, 5 is inclusive
    println!("Success!");
}

fn exercise_2(){
    let x = 1_000.000_1; // What is the type ?
    let y = 0.12; // f23 or f64
    let z = 0.01_f64; // Is it correct 
    let z_1 = 0.9 as f64; // Is it correct

    println!("The Type of x is {}", type_of(&x));
    println!("The Type of y is {}", type_of(&y));
    println!("The value of z is {}, and its type is {}", z, type_of(&z));
    println!("The value of z_1 is {}, and its type is {}", z_1, type_of(&z_1));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
