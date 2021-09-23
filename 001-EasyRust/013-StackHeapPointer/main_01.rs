#[allow(unused_variables)]

fn main() {
    let my_number = 15; // This is an i32
    let single_reference = &my_number; // this is an &i32
    let double_reference = &single_reference; // this is a &&i32
    let five_reference = &&&&my_number; // this is a &&&&i32
}