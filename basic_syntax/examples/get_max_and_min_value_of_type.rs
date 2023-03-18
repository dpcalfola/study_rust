fn main() {
    println!("\n< u type min max value >");
    u_type_min_max_value();

    println!("\n< i type min max value >");
    i_type_min_max_value();
}

fn i_type_min_max_value() {
    // i8
    println!("min value of {}: {}", "i8", i8::MIN);
    println!("max value of {}: {}", "i8", i8::MAX);

    // i16
    println!("min value of {}: {}", "i16", i16::MIN);
    println!("max value of {}: {}", "i16", i16::MAX);

    // i32
    println!("min value of {}: {}", "i32", i32::MIN);
    println!("max value of {}: {}", "i32", i32::MAX);

    // i64
    println!("min value of {}: {}", "i64", i64::MIN);
    println!("max value of {}: {}", "i64", i64::MAX);
}


fn u_type_min_max_value() {

    // u8
    let _min_u8 = u8::MIN;
    let _max_u8 = u8::MAX;

    println!("min value of u8: {}", _min_u8);
    println!("Max value of u8: {}", _max_u8);

    // u16
    let _min_u16 = u16::MIN;
    let _max_u16 = u16::MAX;

    println!("min value of u16: {}", _min_u16);
    println!("Max value of u16: {}", _max_u16);

    // u32
    let _min_u32 = u32::MIN;
    let _max_u32 = u32::MAX;

    println!("min value of u32: {}", _min_u32);
    println!("Max value of u32: {}", _max_u32);

    // u64
    let _min_u64 = u64::MIN;
    let _max_u64 = u64::MAX;

    println!("min value of u64: {}", _min_u64);
    println!("Max value of u64: {}", _max_u64);
}