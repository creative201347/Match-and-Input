


## [Match Statement](https://doc.rust-lang.org/book/ch06-02-match.html)
Match Operator is a powerful operator which lets you match a particular value with a range of values and execute some operations or code for it. It is similar to switch() in Java.
```rs
let num = 70;
match num {
    1 => println!("one"),
    2 => println!("two"),
    3 | 4 | 5 => println!("3-5"),
    6..=10 => println!("6-10"),
    _ => println!("Invalid"),
}
```
```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Reverse Iterators
```rs
for i in (1..=50).rev() {
    print!("{} ", i);
}
```

## [Getting Input](https://www.geeksforgeeks.org/standard-i-o-in-rust/)
The `read_line()` method of this trait can be used to read data, one line at a time, from a file or standard input stream.
The `stdin()` function returns a handle to the standard input stream of the current process, to which the `read_line` function can be applied. This function tries to read all the characters present in the input buffer when it encounters an end-of-line character.
The Writers in Rust are programs that can write data to a file or an output stream in bytes. The `write()` method is used for this purpose.

**Command Line Arguments** are passed to a program before executing it. They are like parameters passed to functions. CommandLine parameters can be used to pass values to the main() function. The `std::env::args(`) returns the commandline arguments.
```rs
use std::io;
fn main() {
    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();
    print!("Hello, {}", name);
}
```