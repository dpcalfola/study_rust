fn main() {
    for_loop_01();
    println!();

    for_loop_02()
}

fn for_loop_01() {
    let vec: Vec<i8> = (0..10).collect();
    for element in vec {
        println!("element : {}", element);
    }
}

fn for_loop_02() {
    for number in (1..5).rev() {
        println!("countdown : {}", number);
    }
    println!("Lift Off !!");
}
