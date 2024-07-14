pub mod generic;

/// This function runs all the generic tests in the `generics` module.
pub fn run_generic_tests() {
    println!("==========================");
    println!("Running Generic Tests:");
    generic::run_generic_tests();
    println!("==========================\n");
}
