use std::io;
fn main() {
    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();
    print!("Hello, {}", name);
}
