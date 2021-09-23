
fn main() {
    let a = String::from("hello");
    println!("{:?}", a);

    let b = &a;
    println!("{:p}", b);

    let c = a.clone();
    let d = &c;

    println!("{:p}", d);
}