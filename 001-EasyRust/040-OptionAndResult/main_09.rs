fn main() {
    let error_value: Result<i32, &str> = Err("There was an error");
    println!("{:?}", error_value); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "There was an error"', main_09.rs:3:32
    // println!("{}", error_value.unwrap()); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "There was an error"', main_09.rs:3:32
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}