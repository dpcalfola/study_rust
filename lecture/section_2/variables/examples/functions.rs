fn main() {
    print_phase("Print my argument");

    // GCD
    println!("{}", gcd(1024, 992));

    println!("{}", multiple_return_values(true));
}

fn print_phase(phrase: &str) {
    println!("Hello from the function!");
    println!("{}", phrase);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        // Swap: a should be larger then b
        if a < b {
            let temp = a;
            a = b;
            b = temp;
        }
        a = a % b;
    };
    return b;
}


fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}