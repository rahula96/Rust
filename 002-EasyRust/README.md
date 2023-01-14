# Easy Rust

출처 : [https://github.com/Dhghomon/easy_rust/](https://github.com/Dhghomon/easy_rust/)

강의 : [https://www.youtube.com/watch?v=W9DO6m8JSSs&amp;list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE](https://github.com/Dhghomon/easy_rust/)

Rust play-ground : [https://play.rust-lang.org/](https://play.rust-lang.org/)

## [001 Easy Rust in Korean: Intro](https://www.youtube.com/watch?v=W9DO6m8JSSs&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=1)

좀 많이 배워야 쓸수 있는 언어

## [002 Easy Rust in Korean: Comments](https://www.youtube.com/watch?v=x7GlQjh2aSw&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=2)

https://github.com/Dhghomon/easy_rust/#comments

/// <== 3개는 문서화로 페이지를 나올 수 있다고 하는군.

## [003 Easy Rust in Korean: Integers](https://www.youtube.com/watch?v=dEMYR99YIao&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=3)

https://github.com/Dhghomon/easy_rust/#types

> [003-Integers/integer.rs](003-Integers/integer.rs)

```rust
fn main() {
    let my_number: u8 = 100; // i8, u32? 아무것도 없으면 i32
    let my_other_number = 50; // i32

    let thrid_number = my_number + my_other_number;

    println!("{:?}", thrid_number);
    
    // type inference 컴파일러가 어떤 타입인지 알고 있어서 따로 해줄 필요가 없다.
}
```

primitive types(=basic types) : Integer, char....등등

The signed integers are: i8, i16, i32, i64, i128, and isize. 
The unsigned integers are: u8, u16, u32, u64, u128, and usize.

- isize -> 32bit -> i32 가 되고
- isize -> 64bit -> i64 가 된다.

## [004 Easy Rust in Korean: Char](https://www.youtube.com/watch?v=yR33X2Ik9W0&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=4)

> [004-char/char.rs](004-char/char.rs)

```rust
fn main() {
    /* 
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
 */
    /* let my_number: u16 = 8; // i32
    let second_number: u8 = 10;

    // casting = simple type change using 'as'

    let third_number = my_number + second_number as u16; */

    let my_number = 'a' as u8;

    println!("My number is {}", my_number);
}
```

모든 char 크기는 4byte

casting = simple type change

## [005 Easy Rust in Korean: Chars 2](https://www.youtube.com/watch?v=fpDTY7UuPaw&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=5)

> [005-char2/char.rs](005-char2/char.rs)

```rust
fn main() {
/*     println!("Size of a char: {} bytes", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ßß': {}", "ßßßßß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len()); */

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
```

char = 4 bytes
"Hello, world"

rust에서 len()은 byte를 의미한다.

글자수... chars().count();

## [006 Easy Rust in Korean: Floats](https://www.youtube.com/watch?v=qHEFgX-zCSs&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=6)

> [006-float/float.rs](006-float/float.rs)

```rust
fn main() {
  let my_number = 9.; // f64
  let other_number = 9; // i32

  println!("{}", my_number + other_number as f64);
}
```

## [007 Easy Rust in Korean: println!](https://www.youtube.com/watch?v=jpLwve-7Cjg&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=7)

- macro가 super function이라고 생각하면 된다고 했는데, 복잡한 코드를 만들어주는 역할
- function that writes code

> [007-println/println.rs](007-println/println.rs)

```rust
fn give_age() -> i32 {
  42
}

fn main() {
  let my_name = "David";
  let _my_age = 42;
  // println!("My name is {} and my age is {}", my_name, give_age());
  println!("My name is {my_name} and my age is {}", give_age());
}
```

## [008 Easy Rust in Korean: println! 2](https://www.youtube.com/watch?v=f-UCvEs7J3I&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=8)

> [008-println/println.rs](008-println/println.rs)

```rust
fn main() {
  let my_city = "Seoul";
  let year = 2002;
  let population = 9_987_987;
  // println!("The city of {} in {} had a population of {}", my_city, year, population);

  // string interpolation
  println!("The city of {my_city} in {year} had a population of {population}");

  // println!(
  //   "The city of {0} in {1} had a population of {2}",
  //   my_city, year, population
  // );
}
```

## [009 Easy Rust in Korean: semicolon, unit type](https://www.youtube.com/watch?v=jxuNkx4XCXg&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=9)

- semicolon, unit type

> [009-misc/misc.rs](009-misc/misc.rs)

```rust
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
```

## [010 Easy Rust in Korean: functions, code blocks](https://www.youtube.com/watch?v=XfWW4XzT17M&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=10)

- functions, code blocks

> [010-misc/misc.rs](010-misc/misc.rs)

```rust
/* fn give_number(one: i32, two: i32) -> i32 {
  one * two
}

fn main() {
  let my_number = give_number(9, 8);
  println!("{my_number}");
} */

fn give_number(one: u32, two: u32) -> u32 {
  // let multiplied = one * two;
  let multiplied_by_ten = {
    let first_number = 10;
    first_number * one * two
  };

  multiplied_by_ten
}

fn main() {
  let my_number = give_number(9, 1);
  println!("{my_number}");
}
```

## [011 Easy Rust in Korean: mutability and shadowing](https://www.youtube.com/watch?v=uHuMJw73ukg&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=11)

- mutability and shadowing

