
fn say_two<T, U>(statement_1: T, statement_2: U)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    println!("I have two things to say: {} and {}", statement_1, statement_2);
}

fn main() {
    say_two("Hello there!", String::from("I hate sand"));
    say_two(
        String::from("Where is Padme?"),
        String::from("Is she all right?")
    );
}