pub fn learning_loop() {
    let break_at = 5;
    let mut cnt = 0;

    println!("This is Infinite Loop");
    loop {
        // This is infinite loop
        println!("Hello, world!");
        cnt += 1;

        if break_at == cnt {
            break;
        }
    }

    println!("\nThis is While Loop");
    cnt = 0;
    while cnt < break_at {
        println!("while cnt={}", cnt);
        cnt += 1;
    }

    println!("\nThis is For Loop");
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    for ch in &arr {
        println!("for ch={}", ch);
    }
}
