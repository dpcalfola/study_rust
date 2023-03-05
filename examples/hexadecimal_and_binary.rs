fn main() {

    println!("\nHow to covert a number to hexadecimal, binary, decimal");

    println!("\n// Convert Hex to Dec");
    let x: u8 = 0x80;
    println!("0x80 is {}", x);

    println!("\n// Convert Hex to binary");
    println!("0x80 is {:#b}", x); //0b10000000
    println!("0x80 is {:b}", x); //10000000

    println!("\n// Convert Bin to Hex");
    println!("0b00100000 is {:x}", 0b00100000); //20
    println!("0b00100000 is {:#x}", 0b00100000); //0x20
}