fn main() {
    let x = &[1, 2, 4]; // bollowed variable
    let mut iterator = x.iter();

    println!("{:?}", iterator);

    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>(); // borrow
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>(); // move

    // println!("vector1 : {:?}", vector1);
    println!("vector1_a : {:?}", vector1_a);
    println!("vector1_b : {:?}", vector1_b);

    let mut vector2 = vec![10, 20, 30];
    vector2.mut

    // let mut vector_2 = vec![10, 20, 30];
    
}