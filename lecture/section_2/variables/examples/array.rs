fn main() {
    let arr = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);
    println!("arr[2]: {}", arr[2]);

    let arr2 = [4, 5, 6];
    println!("arr2[0]: {}", arr2[0]);

    // arr2[0] = 10; // arr2 is not mutable
    println!("arr2[0]: {}", arr2[0]);


    let mut arr3 = [7, 8, 9];
    arr3[0] = 10;
    println!("arr3[0]: {}", arr3[0]);
    
}