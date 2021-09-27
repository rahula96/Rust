fn main() {
    let my_number = 9;
    let references = &my_number;

    // println!("{}", my_number == references); // error[E0277]: can't compare `{integer}` with `&{integer}`

    // expressions은 auto dereference 되지 않는다.
    println!("{}", my_number == *references); // * dereference
}