fn main(){
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Checking the Array
    assert_eq!(arr.len(), 5);

    // Print
    println!("Array = {:?}", arr);


    // We can assign similar value like this:
    let arr1:[i32; 10] = [24; 10]; // Fill 24, 10 element
    println!("Fill = {:?}", arr1);

    // Auto type infer
    let arr2: [_; 3] = [10, 20, 30];
    println!("Auto Type Infer = {:?}", arr2);

    // Auto type infer and size infer
    let arr3 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];                               
    println!("Auto Infer = {:?}", arr3);

    // Accessing the Array elementes
    println!("Element 1 = {}, Element 2 =  {}", arr[0], arr.get(1).unwrap());

    println!("Success!");
}
