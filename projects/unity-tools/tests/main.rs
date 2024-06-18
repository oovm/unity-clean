#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let out = bubble_sort(&mut vec![2, 3, 5, 4]);
    println!("{:?}", out);
}
