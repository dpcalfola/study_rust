fn main() {
    control_flow(27)
}

fn control_flow(i: i32) {
    let answer;
    if i == 1 {
        answer = "The value is one";
    } else if i > 50 {
        answer = "The value is grater than 50"
    } else if i < 25 {
        answer = "The value is less than 25"
    } else {
        answer = "The value is grater than 25 but less than 50"
    }
    println!("{}", answer);
}