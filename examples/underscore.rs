fn main() {
    // Use underscore to improve readability !!
    let one_billion: u64 = 1_000_000_000;
    println!("\nThe variable that holds a value of '1_000_000_000' represents {}", one_billion);

    let num_to_str = one_billion.to_string();
    let num_of_zero = num_to_str.matches('0').count();
    println!("The number of zeros in {} is {}", 1_000_000_000, num_of_zero);
}