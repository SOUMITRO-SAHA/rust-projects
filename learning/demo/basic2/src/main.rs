use basic2::front_of_house;

fn main() {
    // Hosting
    let s: String = front_of_house::hosting::seat_at_table();
    println!("{}", s); // Output: "sit down please"

    // Calling `add_to_waitlist()`
    front_of_house::hosting::add_to_waitlist();

    // Serving

    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
    front_of_house::serving::complain();

    println!("Hello, world!");
}
