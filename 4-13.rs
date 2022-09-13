fn main() {
    let v = [1, 2, 3, 4, 5];
    let result = v.iter().take(3);
    result.for_each(|x| print!("{},", x));
}
