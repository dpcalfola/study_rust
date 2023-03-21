// Vector : resizable array

fn main() {

    // Make vector using macro
    let mut nums = vec![1, 2, 3];

    nums.push(4);
    // println!("{}", nums); // not works

    //
    println!("{:?}", nums); // [1, 2, 3, 4]
    println!("{:#?}", nums); // Vertically expended shape

    // Vector.pop()
    println!("{}", nums.pop().unwrap()); // 4
    println!("{:?}", nums); // [1, 2, 3]


    //
    let mut vec = Vec::new(); // vec!
    vec.push("test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);


    // vec2
    let vec2 = Vec::<i32>::with_capacity(2); //
    println!("vec3: {:?}", vec2);
    println!("vec2.capacity(): {}", vec2.capacity());
    
    // vec3
    let vec3: Vec<i32> = (0..5).collect();
    println!("vec3: {:?}", vec3);
    
}