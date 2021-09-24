fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

fn main() {
    let country = String::from("Austria");

    let country = print_country(country);
    // country = print_country(country); // error[E0384]: cannot assign twice to immutable variable `country`

    print_country(country);
}