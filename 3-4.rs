fn main() {
    for i in 1..10 {
        if i == 4 || i == 0 {
            continue;
        }
        if i == 6 {
            break;
        }
        println!("i : {}", i);
    }
}
