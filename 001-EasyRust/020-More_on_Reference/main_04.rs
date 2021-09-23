fn main() {
    let a = 1; // primitive type == stack
    let b = &a;

    println!("{:p}", b);
    println!("{}", b);

    let c = a;
    let d = &c;

    println!("{:p}", d);
    // println!("{:?}", &a);
    println!("a: {}", a);

}