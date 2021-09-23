아래 YouTube동영상과 Book을 보면서 학습

- 출처 : https://www.youtube.com/watch?v=K4skKgIokmU&list=PLlSZlNj22M7RSK23wBRLdgu8tdFbbMUqR&index=8
- 출처 : https://dhghomon.github.io/easy_rust



### [러스트] 008 - CPU와 Memory vs. Type과 Process

Rust is a language for mission critical tasks such as banking, missle launch, tesla robots.

Rust is different from all other language is safety first.

Fast ==> 최적화 == Optimization == Resource(cpu, memory)를 매우 잘 활용해서, 낭비가 없다.
CPU를 잘 활용 ==> Process, Thread, Blocking, Non-Blocking, Asynchronous, Synchronous, Parallel, Concurrency

Safety ==> 다른 언어에 비해서 ?

### [러스트] 009 - 9.Type inference(왼손은 거들 뿐)

in Rust, Everything is a data, and every data has its type

1. We Designate the type of our variable, argument, constant, function...
1. Rust determines each type of each variable...

All types of our data in our codes are detemined by compiler, we just help the compiler.

let small_numver : u8 = 10;
let : we want a variable
small_number : the name of this variable
: ==> type of this variable follows.
u8 ==> type
= ==> binding is to link data and its name(identifier)
10 ==> data
; ==> statement means no return values.

Expression returns some data

### [러스트] 010 - 10. Printing 'hello, world!'

println!("Hello, World!");
println! // Macro is a code to generate some more codes


Function Signature : fn number() -> i32


fn 
multiply
(
    number_one: i32,
number_two: i32){


### [러스트] 011 - Display and debug

```
error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> main_01.rs:4:41
  |
4 |     println!("This will not print: {}", doesnt_print);
  |                                         ^^^^^^^^^^^^ `()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

() ==> Data Type
'std::fmt::Display' 

When a data type implement the specific trait, then the data type can use the functionality

### [러스트] 012 - Mutability(changing)

Mutable(변할 수 있는) <==> Immutable(변할 수 없는..)

Mutant :  변종

Rust is basically immutable-style language.

Immutable 장점 : Data Safety 높다.
Mutable : Data Safety 낮다.

expected integer, found `&str`

### [러스트] 013 - The stack, the heap, and pointers

1. Stack Data
    - Compile Time
1. Heap Data
    - Runtime 

### [러스트] 014 - 14. More about printing(1/2)

[u8; 27]` doesn't implement `std::fmt::Display

[u8; 27] : u8이 27개
` doesn't implement `
std::fmt::Display : Standard Library's Format module's Display trait

Trait is something which types can implement.

Type is 공간적 대상..... : i32, u8
Each type can do something specific to the type.
일련의 타입들이 할 수 있는 공통적인 동작을 모아서 하나의 그룹을 표현 : Trait

### [러스트] 015 - 14. More about printing(2/3)

