fn main() {
    let mut vector2 = vec![10, 20, 30];

    println!("{:?}", vector2);

    vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", vector2);
}