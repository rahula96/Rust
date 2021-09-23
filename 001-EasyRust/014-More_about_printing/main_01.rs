
fn main() {
    // println!("{:?}", b"This will look like numbers"); // b : bytes로 표시하거라, :? 디버깅을 표현할 수 있다.
    // println!("{}", b"This will look like numbers"); // doesn't implement `std::fmt::Display
    // println!("{:?}", "This will look like numbers"); // doesn't implement `std::fmt::Display
    println!("{}", "This will look like numbers"); // doesn't implement `std::fmt::Display
}