fn main() {
    let v1 = [1, 2, 3, 5, 0];
    let v2 = [4, 5, 6, 7, 6];
    let result: Vec<i32> = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("{:?}", result);
}
