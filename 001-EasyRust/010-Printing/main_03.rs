// #[warn(unused_variables)]

fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} tiems {} is {}", number_one, number_two, result);
    result
}
fn main() {
    let _multiply_result = multiply(8, 9);
}