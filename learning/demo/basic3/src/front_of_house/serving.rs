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
