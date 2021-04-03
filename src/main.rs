fn main() {
    let mut s = "hello".to_string();
    let ss = &mut s;
    ss.push_str(", world");
    println!("{}", s);
}
