fn main() {
    let string_1 = "Hello".to_string();
    let answer = concat_string(string_1);
    println!("{}", answer);
}

fn concat_string(mut s: String) -> String {
    s += "World";
    return s;
}