# Debug & Display

- Types which want to be printable Debug or Display traits must be implemented
- Automatic implementations are only provided for types in the standard library
- Debug trait can be implemented simply by using derivable traits
- Display trait must be manually implemented

## `println!` and `format!`

Printing is handled by a series of `[macros]`. `[macros]` defined in `[std::fmt]` Some of which include:

- `format!`: Write formatted text to `[String][String]`
- `print!`: same as `format!` but the text is printed to the console `(io::stdout)`
- `println!`: same as `print!` but a newline is appended at the end of the line
- `eprint!`: same as `format!` but the text is printed to the standard error `(io::stderr)`
- `eprintln!`: same as `eprint!` but a newline is appended at the end of the line

All parse text in the same fashion. As a plus, Rust checks format correctness at compile time.
