fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {x}");
}

fn main() {
    let x = 5;
    print_one(&x); // Passing a reference to the variable x

    println!("`Main Function`: x is {x}");
}
