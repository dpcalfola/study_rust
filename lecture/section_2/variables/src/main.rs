fn main() {
    let x = 5;
    println!("The value of x is {x}");

    const CONST_VALUE: i32 = 32;
    println!("const keyword const value name should be Upper case {}", CONST_VALUE);


    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal); // 255
    println!("{}", hex); // 255
    println!("{}", octal); // 255
    println!("{}", binary); // 255

    let y = 2.0;
    println!("y = {}", y);

    let t = true;
    let f: bool = false;
    println!("boolean: {} {}", t, f);

    let c = 'c'; // Single quote means char
    let s = "s"; // Double quote means String
    println!("{} {}", c, s);

    let a = 10;
    let b = 4;
    let reminder = a % b;
    println!("reminder:  {}", reminder);
}
