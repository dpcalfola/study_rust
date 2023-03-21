fn main() {
    let tup_01 = (500, "hi", true);
    println!("tup_01.0: {}", tup_01.0);
    println!("tup_01.1: {}", tup_01.1);
    println!("tup_01.2: {}", tup_01.2);

    let (x, y, z) = tup_01;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

}