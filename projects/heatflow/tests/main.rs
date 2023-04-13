use heatflow::Circular;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn keep_size() {
    assert_eq!(std::mem::size_of::<Circular<()>>(), 24);
    assert_eq!(std::mem::size_of::<Circular<i128>>(), 24);
}

#[test]
pub fn test() {
    let mut queue = Circular::<i128>::new(5);
    println!("{:?}", queue);
    for i in 1..=10 {
        queue.push(i);
        println!("[{}, {}, {}, {}, {}]", queue.get(0), queue.get(1), queue.get(2), queue.get(3), queue.get(4));
    }
    println!("oldest: {}", queue.oldest());
    println!("newest: {}", queue.newest());
}