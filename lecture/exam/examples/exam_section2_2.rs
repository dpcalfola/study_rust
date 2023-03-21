fn main() {
    let mut vec_1 = vec![2, 4, 6, 8, 10];
    vec_1.pop();
    vec_1.push(12);

    println!("{:?}", vec_1);
}