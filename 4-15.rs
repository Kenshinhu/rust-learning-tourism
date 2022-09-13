fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7];
    let result = v.iter().rev();
    result.for_each(|x| print!("{},", x));
}
