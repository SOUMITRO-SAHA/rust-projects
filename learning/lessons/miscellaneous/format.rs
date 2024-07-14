fn main() {
    let s1: &str = "hello";
    let s: String = format!("{}, world!!!", s1);

    assert_eq!(s, "hello, world!!!");

    println!("{}", s); // Output: "hello, world!!!"
}
