# Tuple

- Way to store related pieces of information in a single variable
- Collection of values of **different types** grouped together as a single **Compound value** (type composed of other types)
- Stored as a fixed-size contiguous block of memory on the stack
- Signature is (T1, T2, T3...), where T1, T2, T3 are the types of tuple's members.
- Members can be extracted from the tuple using indexing

```rs
let _t0: (u8, i16) = (0, -1);

let _t1: (u8, (i16. u32)) = (0, (-1, 1));

let _t2: (i32, &str, String) = (24, "Soumitra Saha", String::from("Software Engineer"));

// Accessing 
println!("{}", _t2.0); // 24
println!("Success");
```

- Long tuples can't be printed

```rs
let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);

println!("{:?}", too_long_tuple)
```

- Destructuring tuple with pattern

```rs
fn main(){
    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    // Destructuring tuple with pattern
    let (x, y, z) = tup;

    println!("1st => {}; 2nd => {}; 3ed => {}.", x, y, z);
}
```

- Tuples can be used as function arguments and return values

```rs
fn main(){
    let (x, y) = sum((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success");
}

fn sum(nums:(i32, i32)) -> (i32, i32){
    (nums.0 + nums.1, nums.0 * nums.1)
}
```