use std::collections::HashMap;
fn main() {
    let mut m = HashMap::<usize, usize>::new();
    m.insert(3, 5);
    m.insert(4, 5);
    for (x, y) in m.iter_mut() {
        println!("{} {}", x, y);
    }
    for (x, y) in m.iter().sorted_unstable_by(|x, y| Ord::cmp(x, y)) {
        *y = 3;
    }
    for (x, y) in m.iter_mut() {
        println!("{} {}", x, y);
    }
}
