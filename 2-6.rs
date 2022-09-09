fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5];
    let arr3: [i32; 5] = [1; 5];
    let arr4 = [1; 5];

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);
    println!("arr4: {:?}", arr4);

    println!("arr1[0]: {}", arr1[0]);
    println!("arr3[2]: {}", arr3[2]);
    println!("arr1[2]: {}", arr1[2]);
}
