fn give_age() -> i32 {
  42
}

fn main() {
  let my_name = "David";
  let _my_age = 42;
  // println!("My name is {} and my age is {}", my_name, give_age());
  println!("My name is {my_name} and my age is {}", give_age());
}
