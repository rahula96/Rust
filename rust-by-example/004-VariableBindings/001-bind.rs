fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let _copied_integer = an_integer;

    println!("An integer: {:?}", _copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
}