use heatflow::Circular;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn keep_size() {
    assert_eq!(std::mem::size_of::<Circular<5, ()>>(), 16);
    assert_eq!(std::mem::size_of::<Circular<5, i128>>(), 16);
}

#[test]
pub fn test() {
    let mut queue = Circular::<8, i128>::new();
    println!("{:?}", queue);
    for i in 1..10 {
        queue.push(i);
        println!("{:?}", queue);
    }
    println!("oldest: {}", queue.oldest());
    println!("newest: {}", queue.newest());
}