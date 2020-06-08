fn main() {
    let mut v = vec!["hello".to_string(), "world".to_string()];
    for item in v.iter() {
        v.remove(0);
        println!("{}", item);
        return;
    }
}
