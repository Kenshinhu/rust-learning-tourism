use std::collections::VecDeque;

fn main() {
    let mut v: VecDeque<u32> = VecDeque::with_capacity(10);
    v.push_back(0);
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_back(4);
    v.push_back(5);
    v.push_back(6);
    v.push_back(7);
    v.push_back(8);
    v.push_back(9);
    v.push_back(10);

    println!("{:?}", v.remove(10));
    println!("{:?}", v);
}
