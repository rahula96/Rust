fn main() {
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("three_to_five: {:?}", three_to_five);
    println!("start_at_two: {:?}", start_at_two);
    println!("end_at_five: {:?}", end_at_five);
    println!("everything: {:?}", everything);
}