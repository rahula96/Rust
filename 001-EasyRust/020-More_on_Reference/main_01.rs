fn main() {
    let country = String::from("Austria"); // 
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_one);
    println!("ref_one{:p}", ref_one);
    println!("&country{:p}", &country);
    println!("{:?}", ref_one);
}