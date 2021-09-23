
fn main() {
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    println!("{}", together);

    let a = String::from("This is the string text"); // 24bytes
    let b = "this is the string text".to_string(); // 24bytes
    let my_string: &str = "Try to make this a String".into();

    println!("{}", a);
    println!("{}", b);
    println!("{}", my_string);


}