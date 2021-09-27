
struct Item {
    number: u8,
}

fn main() {
    let item = Item { number: 8 };

    // let reference_number = &item.number; // reference number type is &u8
    let reference = &item;

    // println!("{}", reference_number == 8); // error[E0277]: can't compare `&u8` with `{integer}`
    // println!("{}", *reference_number == 8); // OK : dereference
    // println!("{}",item.number == 8); // OK
    println!("{}",reference.number == 8); // OK

}