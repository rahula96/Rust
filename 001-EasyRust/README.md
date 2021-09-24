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


### [러스트] 016 - Stack, Heap, Own, Borrow

Rust is fast, safe, and concurrent language

- Fast : CPU/Memory
- Safe : CPU/Memory

- Stack,Heap : Fast
- Own, Borrow : Safe

Data Type(i32, i8....) is for fast.

C/C++, Java, Go ... => Unsafe할수 있다고 말하네....갸우뚱...

Rust => Human cannot produce unsafe codes.

- Owned Data(name)
    1. 모든 데이터는 어느 한 시점에 반드시 하나의 꼭 하나의 소유자(Owner)를 갖는다. 그 소유자는 주로 Variable
    1. 모든 데이터는 만들어지는 시점과 소멸되는 시점이 반드시 한 번씩만 있다.
    1. 모든 데이터의 소유권은 어느 한 소유자에서 다른 소유자로 이전 될 수 있다.
    1. 모든 데이터는 매우 특수한 상황에서 공동 소유자가 존재할 수 있다.
    
    String

- Borrowed Data(&name or ref name)
    1. 다른 소유자에게서 빌려온 데이터
    1. 어느 한 소유자는 자신의 데이터를 무한히 반복하여 빌려줄 수 있다.
    1. 빌려주는 사람(소유자)는 반드시 빌려간 사람 보다 오래 살아야 한다.

    &str


### [러스트] 017 - 15. Strings(1/3)
&str : Borrow = Not Owned == 사용한 이후에는 다시 채워둘 것, 무한 반복해서 소비할 수 있다.
1. 데이터가 Stack에 할당
1. 이 데이터를 무한 반복해서 재사용
1. 이 데이터의 소유자는 불특정 : 누구나 가져가서 쓸수 있다.


std::string::String
1. Heap에 저장된 Data
1. 데이터의 소유자가 특정
1. 어느 한 시점, 반드시 하나의 꼭 하나의 소유자가 있다.

Rust는 Misstion Critical Language : 자원(Resource)을 매우 정밀하게 사용, 활용, 관리해야 하는 언어

### [러스트] 018 - 16. Strings(2/3)

Sized
    - String 데이터 타입이 Stack에서 차지하는 공간의 크기는 특정 ==> 24bytes

Not-Sized => Various Sized
    - &str 데이터타입이 Stack에서 차지하는 공간의 크기는 Argument에 의해 결정

### [러스트] 019 -15. Strings(3/3)


### [러스트] 020 - 17. More on references

### [러스트] 020 - (보충 설명) 메모리 구조 - Stack, Heap, Own, Borrow(1/2)

1. Some data type are stored in Stack. Others are in Heap. The former are numbers(i32, i64..) and tuple, array... The latter are complex data types such as struct and Vec

### [러스트] 021 - 18. Mutable references(1/2)


### [러스트] 022 - 18. Mutable references(2/2)

### [러스트] 023 - 24. Copy types(1/2)

Some types(=primitive type) just copy original value to a new variable.

### [러스트] 024 - 25. Copy types(2/2)

### [러스트] 025 - 19.Giving references to functions

### [러스트] 026 - 21. Collection types(1/2)

### [러스트] 028 - 21. Collection types(2/2)
