use std::io;

fn main() {
    // read_line
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");

    println!("input String is {}", input);
}