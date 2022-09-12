fn main() {
    let add_one = |x: u32| -> u32 { x + 1 };

    let add_one_v2 = |x| x + 1;

    println!("add one by v1 : {}", add_one(1));
    println!("add one by v2 : {}", add_one_v2(1));
}
