use std::mem::size_of_val;

fn main(){
    /* charecters */
    let c1 = 'a';
    println!("The default size of the char at 64-bit system is {}", size_of_val(&c1));

    let c2 = '🏯';
    println!("The default size of the char at 64-bit system is {} & the charecter is {c2}", size_of_val(&c2));

    /* Boolean */
    // Boolean holds `true` & 'false'  of size 1 byte


    /* Unit Type */
    // It is `()` - Empty tuple of size 0 bytes, used to return "nothing" in expressions or
    // functions

    /* String + &str */
    let sl:&str = "SOUMITRA SAHA";
    take_string_literal(sl);

    let s1 = String::from("Hi, How are you??");

    take_string_literal(s1.as_str()); // Converting String -> &str
    take_string_literal(&s1); // Refer: &String -> &str
    
    greeting(sl.to_string()); // Converting &str -> String
    greeting(s1);// Passing the String Directly, basically the ownership will transfered here
}

fn take_string_literal(sl : &str){
    println!("This is the passed string literal {sl}");

    // Re Assigning the String Literal to another variable
    let new_sl = sl;

    // Now Printing the new_sl
    println!("The new_sl is {new_sl}");
}

fn greeting(msg: String) {
    println!("{msg}");
}
