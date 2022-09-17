// () - empty tuple, unit type (void)

fn number() -> i32 {
  // 8; // implicitly returns `()` as its body has no tail or `return` expression
  8
}

fn empty_tuple() -> () {
  8;
}

// ";"이 있으면 empty tuple을 리턴한다.
// fn main() -> i32 {
fn main() {
  // let x = 9;
  // let my_number = number();

  let tuple = empty_tuple();
  // tuple;
  // 6

  // println!("{tuple}"); // cannot be formatted with the default formatter
  println!("{tuple:?}");
}
