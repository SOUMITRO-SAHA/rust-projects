# Closures

- **Anonymous functions** that are able to **capture** values from the scope in which they are defined
- Can be defined **inline** (for example as function parameter)
- Don't require type annotations
- Can take ownership of a value by using **move** keyword

> In JS called `arrow functions` and in python called `lambda function`

## Fn Traits

- Trait that defines **signature** for closures/functions
- Describes types, number of arguments and return tyupe
- Three different traits
  - **FnOnce**
    - Closures that can be called once
    - Takes ownership of capture values
  - **FnMut**
    - Might mutate captured values
    - Can be called more than once
  - **Fn**
    - Doesn't take ownership of captured values
    - Doesn't mutate anything
    - Might not even capture anything from its environment

## Fn, FnMut, FnOnce

When taking a closure as an input parameter, the closure's complete type must be annotated using one of the following traits:

- Fn: the closure uses the captured value by reference `(&T)`
- FnMut: the closure uses the captured value by mutable reference `(&mut T)`
- FnOnce: the closure uses the capture value by value(T)

## Example

```rs
fn main() {
    let x = 1;
    let closure = | val | val + x;
    assert_eq!(closure(3), 4);
}
```

- This closure **captures** the value of x and **modifies** it. Compiler will capture variables in the **least restrictive** manner possible.
- In this case a **mutable reference of x** is taken, rather than taking ownership because it's less restrictive.


