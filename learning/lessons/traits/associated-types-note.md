# Associated Types

- Allow to specify a type that is associated with the trait
- When implementing the trait for a specific type we have to specify the concrete type
- Basically a type placeholder that the trait methods can be use in their signature
- Similar to generic types but are more flexible because they allow a trait to have different associated type for different implementing types

```rs
trait MyTrait {
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct {}

impl MyTrait for MyStruct{
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}
```

- Here we define a trait that has an associated type a method that returns a value of this type.
- When implementing the trait for a specific type (`MyStruct`), then we have to give the associated `type MyType` a concrete type, in this case `i32`
