
fn main() {
    let mut my_number = 8; // default i32
    // my_number = 10;
    let num_ref = &mut my_number;

    // let a = num_ref + 1; // error[E0369]: cannot add `{integer}` to `&mut {integer}`
    let a = *num_ref + 1; // error[E0369]: cannot add `{integer}` to `&mut {integer}`

    println!("{}", a);
    // println!("{}", my_number);
    println!("{:p}", num_ref); // usize(64bit)
    // println!("{}", num_ref);
}