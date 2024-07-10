pub fn learning_vector() {
    // Method 1
    let mut v1: Vec<i32> = Vec::new(); // declaration of vector

    v1.push(1);
    v1.push(2);
    v1.push(3);

    println!("Method 1: v1={:?}", v1);

    // Method 2
    let v2 = vec![10, 20, 30, 40, 50, 60];

    println!("Method 2: v2={:?}", v2);
}
