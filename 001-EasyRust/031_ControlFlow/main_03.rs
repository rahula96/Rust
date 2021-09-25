fn main() {
    let my_number = 5;
    let second_numer = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };

    println!("second_numer: {}", second_numer);
}