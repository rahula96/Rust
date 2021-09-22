
fn main() {
    // let doesnt_print = 1; // primitive data type
    let doesnt_print = (); // non primitive data type
    // println!("This will not print: {}", doesnt_print); // error
    println!("This will not print: {:?}", doesnt_print); // error
}