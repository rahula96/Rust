fn main() {
    let number = 9; // value, Primitive Type(number, Char ())
    let number_ref = &number;

    println!("{:p}", number_ref); // pointer
    println!("{:?}", number_ref);
    println!("{}", number_ref);
}