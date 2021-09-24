
fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");

    println!("{:p}", &country);
    println!("{}", &country);

    print_country(country.clone());
    print_country(country);
}