fn main() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }
}