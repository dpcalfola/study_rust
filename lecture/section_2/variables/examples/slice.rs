fn main() {
    let v: Vec<i32> = (0..5).collect();

    let sv = &v; // reference
    println!("sv: {:?}", sv);

    let sv_1: &[i32] = &v;
    println!("sv_1: {:?}", sv_1);

    let sv_2 = &v[2..4];
    println!("sv_2: {:?}", sv_2);
}