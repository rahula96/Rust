아래 YouTube동영상과 Book을 보면서 학습

출처 : https://www.youtube.com/watch?v=K4skKgIokmU&list=PLlSZlNj22M7RSK23wBRLdgu8tdFbbMUqR&index=8
출처 : https://dhghomon.github.io/easy_rust



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