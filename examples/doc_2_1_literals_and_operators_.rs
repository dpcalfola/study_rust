fn main() {
    integer_operations();
    bitwise_operations();
    use_underscores_to_improve_readability()

    // Reference: https://doc.rust-lang.org/rust-by-example/primitives/literals.html
}

fn use_underscores_to_improve_readability() {
    println!("One million is written as {}", 1_000_000u32);
}


fn integer_operations() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // => An unsigned integer value should be positive, as otherwise it would result in an overflow

    // Compile Error: Overflow
    println!("1 - 2 = {}",1u32 - 2 );

    // It is OK
    println!("1 + 2 = {}", 1i32 + 2);
}

fn bitwise_operations() {
    println!("Bitwise operations");
    println!("CASE1: 0011 AND 0101 is {:#04b}", 0b0011u32 & 0b0101);
    println!("CASE2: 0011 OR 0101 is {:#04b}", 0b0011u32 | 0b0101);
    println!("CASE3: 0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    // 0b00000001 << 5 => 0b00100000 == 32(decimal)
    println!("CASE4: 1 << 5 is {}", 1u32 << 5); //32

    // 0b10000000 >> 2 => 0b00100000 == 32(decimal) ==
    println!("CASE5: 0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //0x20
}