fn main() {
    let mut count = 0;
    let mut counter = 0;

    while count != 10 {
        count += 1;
        counter = count * 2;
        println!("count: {} , counter: {}", count, counter);
    }

    println!("finally! count: {}, counter: {}", count, counter);
}
