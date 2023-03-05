fn main() {
    let mut cnt = 0;


    loop {
        if cnt == 100 {
            break;
        }
        cnt += 1;

        if cnt % 2 == 0 {
            println!("{cnt} is even number");
            continue;
        }

        if cnt % 2 == 1 {
            println!("{cnt} is odd number");
            continue;
        }
    }
}
