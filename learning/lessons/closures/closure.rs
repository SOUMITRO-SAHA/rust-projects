fn main() {
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(3), 4);
    println!("{}", closure(3));
}
