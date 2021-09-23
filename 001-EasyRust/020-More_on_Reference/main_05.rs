
fn main() {
    let a = String::from("hello");
    let b = &a;

    println!("{:p}", b); // ?

    let e = a;
    let f = &e;

    println!("{:p}", f); // ? 
}