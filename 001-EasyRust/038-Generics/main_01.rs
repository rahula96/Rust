fn return_number(number: u32) -> i32 {
    println!("Here is your number: {}", number);
    // number // error[E0308]: mismatched types 
    // expected `i32`, found `u32`
    number as i32
}

fn main() {
    let number = return_number(5);

    println!("{}",number);
}