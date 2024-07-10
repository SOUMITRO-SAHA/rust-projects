fn main(){
    /* Statements & Expressions */
    let x = 2;
    let y = {
        let x_square = x * x; 
        let x_cube = x * x * x;

        x_square + x_cube + x // this is evaluating a value and does not ends with a semi-colon,
                              // This is an expression  
    };

    let z = {
        8 * x + y
    };

    println!("The value of y is {y}");  
    println!("The value of z is {z}");  
}
