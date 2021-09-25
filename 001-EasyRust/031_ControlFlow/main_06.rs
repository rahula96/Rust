fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{} is an luchy number....", number),
        number @ 13 => println!("{} is unlucky in North America", number),
        _ => println!("Looks like a normal number"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(4);
}