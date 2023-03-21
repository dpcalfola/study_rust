// 마지막 라인은 세미 콜론 생략 가능

fn main() {
    if_syntax();
    loop_syntax()
}

fn if_syntax() {
    // if syntax
    let one = 1;
    if one > 10 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("False");
    }
}

fn loop_syntax() {
    println!("Loop!");

    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 3 {
                break;
            }
            if num == 2 {
                // break to outside loop
                break 'counter;
            }
            decrease -= 1
        }
        num += 1;
    }
}
