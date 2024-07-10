# Array

- Fixed-size collection of elements of the same data type stored as contiguous block in stack memory.
- Signature of array is [T, Length] which indicates that the **length is fixed** at compile time
- Arrays can neither grow nor shrink, they must **retain their size**.

## Accessing the Array elements

```rs
fn main() {
    let arr = [1, 2, 3];

    // Method-1
    println!(arr[0]);

    // Method-2
    println!(arr.get(0).unwrap());
}
```

- `get` returns an `Option<T>`, it's safe to use. But, indexing is not safe.

## Slice

- Reference to contiguous sequence of elements in a collection
- Provide a way to **borrow part** of a collection without taking **ownership** of the entire collection
- Can be created from arrays, vectors, Strings and other collections implementing the Defer trait.

```rs
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[0..3]; // 0 & 3 are the index; 3 will be excluded.
// We can also write this as [..3] => It will starts form 0 index

assert_eq!(slice, &[1, 2]);
```

- slice has the type `&[i32]` in this example.
- Works like string slices do, by storing a reference to the first element and a length.

- a slice reference is a two-word object, for simplicity reasons, form now on we will use slice instead of `slice reference`. The first word is a pointer to the data, and the second word is the length of the slice. The word size is the same as `usize`, determined by the processor architecture, e.g. bits on an `x86-64`. Slice can be used to borrow a section of an array, and have the type signature `&[T]`.

### **This also will work for a String or String Literal**

```rs
let s: String = String::from("hello world");
let sl: &str = "Hello, world!";

let slice_1: &str = &s[..4];
let slice_2: &str = &sl[..4];

println!("{:?}", slice_1)
println!("{:?}", slice_2)
```
