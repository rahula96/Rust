
// error[E0106]: missing lifetime specifier
fn return_str() -> &str { // return type is a borrow type, 
    let country = String::from("Austria");
    let country_ref = &country;
    
    println!("{}", country_ref);

    country_ref
}

fn main() {
    let country = return_str(); // dangling

    println!("{}", country);
}