
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: std::fmt::Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}

fn main() {
    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };

    let numer = 55;

    print_item(charlie);
    print_item(numer);
}