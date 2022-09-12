fn say_hello() {
    println!(" Hello Function pointer!");
}

fn main() {
    let fn_ptr: fn() = say_hello;
    println!("{:p}", fn_ptr);

    let other_fn = say_hello;
    // println!("{:p}", other_fn);

    other_fn();
    fn_ptr();
}
