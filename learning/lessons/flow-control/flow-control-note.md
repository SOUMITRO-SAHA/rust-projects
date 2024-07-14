# Flow Control

- Normal flow of the program: **Top to bottom**, **line by line**
- Concept that refers to ability to control the order in which statements or instructions are executed in a program
- Allows to specify which instructions should be executed under which conditions and in what order
- **Conditions**
  - if/else
  - match
- **Loops**
  - for/while/loop
  - continue/break

```rs
fn main(){
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}
```

- If/else expression can be used in assignment

```rs
fn main() {
    let n: i32 = 5;
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        10 * n
    } else {
        println!(", and is a big number, halve ten-fold");

        n / 2
    }

    println!("{} -> {}", n, big_n);
}
```

- The for in construct can be used to iterate through an Iterator, e.g. a range a..b

```rs
fn main() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}
```

- The **While** keyword can be used to run a loop while a condition is true

```rs
fn main() {
    let mut n:i32 = 1;

    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz!");
        } else if n % 3 == 0 {
            println!("fizz!");
        } else if n % 5 == 0 {
            println!("buzz!");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}
```

- Continue and break
  - Use **break** to break the loop
  - **continue** will skip over the remaining code in current iteration and got to the next iteration

- **Loop**
  - Loop is usually used together with break or continue
  - Loop is an expression, so we can use it with **break** to return a value
  - It is possible to break or continue outer loops when dealing with nested loops in there cases, the loop must be annotated with some 'label, and the label must be passed to the break/continue statement.

```rs
fn main() {
    let mut cnt = 0;
    'outer: loop { // `outer => label
        'inner1: loop {
            if cnt >= 20 {
                break 'inner; // `break` is also works, this would break only the inner1 loop
            }
            cnt += 2;
        }

        cnt += 5;

        'inner2: loop {
            if cnt >= 30 {
                break 'outer; // this break the outer loop
            }

            // This will continue the outer loop
            continue `outer;
        }
    }

    println!("Success!");
}
```
