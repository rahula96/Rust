
fn return_number<T>(number: T) -> T {
    // println!("Here is your number: {}", number); // error[E0277]: `T` doesn't implement `std::fmt::Display`
    // println!("Here is your number: {:?}", number); // error[E0277]: `T` doesn't implement `Debug`
    println!("Here is your number");

    number
}

fn main() {
    // let number = return_number(8);
    let number = return_number("dskfajdflkjads;lf");

    println!("{}", number);
}