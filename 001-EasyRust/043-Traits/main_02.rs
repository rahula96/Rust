
struct Cat {
    name: String,
    age: u8,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("{}", mr_mantle);
}