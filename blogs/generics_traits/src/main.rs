mod generics;
mod traits;

fn main() {
    // Run generic tests
    generics::run_generic_tests();

    // Run trait tests
    traits::run_traits_tests();
}
