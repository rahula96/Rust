
struct Circle {
    radius: i32,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 0 };

    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;

    println!("Sum: {:?}", sum);
}