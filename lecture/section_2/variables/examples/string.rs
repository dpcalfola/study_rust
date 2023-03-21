fn main() {
    let name = String::from("Tyler");
    println!("{}", name);

    let course = "Rust".to_string();
    println!("{}", course);

    let nuw_name = name.replace("tyler", "ty");
    println!("{}", nuw_name);

    // &str = "string slice" or "stir"

    let str1 = "Hello"; // &str
    println!("{}", str1);

    let str2 = str1;
    println!("{}", str2);

    let str3 = &str1; // &&str
    println!("{}", str3);

    let str4 = str1.to_string(); // String
    println!("{}", str4);



    // Compare &str
    println!("{}", "ONE".to_lowercase() == "one");
}