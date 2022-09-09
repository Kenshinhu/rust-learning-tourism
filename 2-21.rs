use std::collections::VecDeque;

fn main() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_back(4);

    println!("e:{}", v[0]);

    println!("e:{}", v[6]);
}
