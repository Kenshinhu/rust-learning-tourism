fn main() {
    let mut count = 0;
    let _counter = loop {
        count += 1;
        let _counter = count * 2;
        println!("count: {}, counter: {} ", count, _counter);

        if count == 10 {
            break _counter;
        }
    };

    println!("final count:{}", count)
}
