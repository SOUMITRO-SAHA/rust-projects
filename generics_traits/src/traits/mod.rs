pub mod trait_bounds_advance;
pub mod trait_bounds_functions;
pub mod trait_bounds_methods;
pub mod trait_bounds_structs;
pub mod traits;

/// This function runs all the trait tests in the `traits` module.
pub fn run_traits_tests() {
    println!("==========================");
    println!("Running Traits Test:");
    traits::run_traits_test();
    println!("==========================\n");

    println!("==========================");
    println!("Running Advanced Trait Bounds Test:");
    trait_bounds_advance::run_advanced_trait_bounds_tests();
    println!("==========================\n");

    println!("==========================");
    println!("Running Basic Trait Bounds Test:");
    trait_bounds_functions::run_basic_trait_bounds_test();
    println!("==========================\n");

    println!("==========================");
    println!("Running Multiple Trait Bounds Test:");
    trait_bounds_functions::run_multiple_trait_bounds_test();
    println!("==========================\n");

    println!("==========================");
    println!("Running Trait Bounds Method Test:");
    trait_bounds_methods::run_trait_bounds_method_test();
    println!("==========================\n");

    println!("==========================");
    println!("Running Trait Bounds Struct Test:");
    trait_bounds_structs::run_trait_bounds_struct_test();
    println!("==========================\n");
}
