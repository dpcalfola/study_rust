fn main() {
    // Variables can be type annotated
    let _logical: bool = true;
    println!("{}", _logical);

    // Type annotation example 2
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 7i32; // suffix annotation
    println!("{} {}", a_float, an_integer);

    // Or a default will be used
    let default_float = 3.0; // `f64`
    let default_integer = 9; // `i32'
    println!("{} {}", default_float, default_integer);

    // A type can also be inferred from context -> (fola) I don't like it
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 42934985792871i64;
    println!("{}", inferred_type);

    // A mutable variable's value can be changed
    let mut mutable = 15; // mut <== mutable
    println!("{}", mutable);

    // Error! The type of a variable can't be changed
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;
    println!("{}", mutable);
}