fn main() {
    let s = "String".to_string();
    let chars: Vec<char> = s.chars().collect();

    println!("{:?}", chars);
}