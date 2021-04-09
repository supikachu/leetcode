fn main() {
    let mut v = vec![1, 2, 3];
    v.iter_mut().for_each(|ele| *ele += 1);
}
