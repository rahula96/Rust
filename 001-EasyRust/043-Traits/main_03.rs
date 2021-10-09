
struct Cat {
    name: String,
    age: u8,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn print_cats(pet:String) {
    println!("{}", pet);
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    print_cats(mr_mantle.to_string());
    // print_cats(mr_mantle); // mismatched types
    // println!("{}", mr_mantle);
    println!("Mr. Mantle's age is {} letters long.", mr_mantle.to_string().chars().count());
}