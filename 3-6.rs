fn match_value(value: Option<i32>) {
    match value {
        Some(7) => println!("Seven"),
        _ => (),
    }
}
// fn if_let_value(value: Option<i32>) {
//     if let Some(7) = value {
//         println!("Seven")
//     }
// }

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    while let Some(value) = vec.pop() {
        println!("{}", value);
    }
}
