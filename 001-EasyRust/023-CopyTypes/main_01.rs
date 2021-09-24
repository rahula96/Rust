
fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Kiribati");
    print_country(country);
    print_country(country); // error[E0382]: use of moved value: `country`
}