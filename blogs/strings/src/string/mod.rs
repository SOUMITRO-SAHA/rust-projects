mod strings;
mod strings2;

use strings::run_string;
use strings2::run_string_2;

pub fn run() {
    println!("------------- [Start Running String 1] ----------------");
    run_string();
    println!("------------- [End Running String 1] ----------------");

    println!("------------- [Start Running String 2] ----------------");
    run_string_2();
    println!("------------- [End Running String 2] ----------------");
}
