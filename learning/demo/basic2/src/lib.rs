pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to the waitlist.");
        }

        pub fn seat_at_table() -> String {
            String::from("sit down please")
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Taking your order.");
        }

        pub fn serve_order() {
            println!("Serving your order.");
        }

        pub fn take_payment() {
            println!("Taking your payment.");
        }

        // Maybe you don't want the guest hearing the your complaining about them
        // So just make it private
        pub fn complain() {
            println!("I'm sorry, I can't hear you complaining.");
        }
    }
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        crate::front_of_house::serving::serve_order();
    }

    pub fn cook_order() {
        println!("Cooking the order.");
    }
}

pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
