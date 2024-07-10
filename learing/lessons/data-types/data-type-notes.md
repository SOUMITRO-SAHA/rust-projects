# Numbers - Integer Types

- Signed integer: Can represent both **positive** and **negative** integers.
- Unsigned integer: Always positive integers

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | i8      | u8       |
| 16-bit  | i16     | u16      |
| 32-bit  | i32     | u32      |
| 64-bit  | i64     | u64      |
| 128-bit | i128    | u128     |
| arch    | i<size> | u<size>  |

##### Numbers - Default Types

- Integer: `i32`
- Floats: `f64`

##### Numbers - Ranges

- Smallest possible 8-bit integer (Unsigned): 0
- Largest possible 8-bit integer (Unsigned): 255

- Smallest 16-bit (Unsigned): 0
- Largest 16-bit (Unsigned): 65,535

| Data Type | Minimum     | Maximum    |
| --------- | ----------- | ---------- |
| i8        | -128        | 127        |
| i16       | -32768      | 32767      |
| i32       | -2147483648 | 2147483647 |
| --------- | ----------  | ---------- |
| u8        | 0           | 255        |
| u16       | 0           | 65535      |
| u32       | 0           | 4294967295 |

#### Numbers - `usize` & `isize`

- Architecture dependent
- On 32-bit architecture: 32-bit
- On 64-bit architecture: 64-bit
- Pointer size integer type, matches size of a word in given platform

**What is a word?**

- In a 32-bit processor it can access 4 bytes (32 bits) at a time.
- In a 64-bit processor it can access 8 bytes (64 bits) at a time.

#### Numbers - Floating Point

- f32 - size of 32 bits
- f64 - size of 64 bits
- Representation according to IEEE-754 specification

### Character

- In rust we use `' '` (single quotes) for character, and `" "` (double quotes) for strings.

### Boolean

- Boolean value of `true` or `false` of size 1 byte

### Unit

- Empty tuple of size 0 bytes, used to return 'nothing' in expressions or functions.

## Compound Data Type

## String vs. `&str`

- A String is a heap-allocated string type that owns its contents and is mutable.
- A `&str` is an immutable sequence of UTF-8 bytes in memory, it does not own the underlying data and is immutable.
- Think of `&str` as a view on a sequence of characters (stored as UTF-8 bytes) in memory.

- Use &str if you just want to a view of a string.
- &str is more lightweight and efficient than String
- User String if you need to own the data and be able to mutate it.

## String Literal (`&str)

- A string literal is a sequence of characters enclosed in double quotes (`"`)
- Fixed size, compile-time known sequence of UTF 8-bytes
- The type is `&'static str`, which indicates the data is stored in static storage, meaning it is valid throughout the entire lifetime of the program.
- The data is **hardcoded** into the **executable** and stored in **read-only memory**, meaning they are **immutable**
