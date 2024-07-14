
# Box

- Smart pointer that allows to store data on the heap rather than the stack
- Use Box when you have a type whose size can't be know at compile time
- Returns a pointer to the data stored on the heap

## `&` vs Box

- **Memory**: Box allocates data on heap and owns it, also responsible for deallocating when value goes out of scope, reference only points to a value already in memory
- **Lifetime**: Box can be passed across scopes, reference has limited lifetime
- **Box**: can be cloned, reference not
- Box can be used in pattern matching
