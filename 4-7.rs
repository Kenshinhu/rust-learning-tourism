fn main() {
    let i = 1;
    let add = |x| x + i;

    let add_v2 = |x: i32| -> i32 { x + i };

    println!("add + {} : {}", i, add(1));
    println!("v2: add + {} : {}", i, add_v2(1));
}
