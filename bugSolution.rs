fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());
    
    //Correct way to check for the end of the iterator
    match iter.next() {
        Some(x) => println!("Third element: {:?}", x),
        None => println!("Iterator is empty")
    };
}