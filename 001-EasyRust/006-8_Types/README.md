# 러스트] 005 - 8. Types( 명사와 동사)

https://youtu.be/m7VG4TfyVHg

https://dhghomon.github.io/easy_rust/Chapter_7.html


- 명사
(1) Primitive Type = Data is in Stack = 숫자, 문자(글자 Char), Tuple....

(2) Complex type = Data is in Heap = 문자열(텍스트, 스트링), Struct, Vector

- 동사
Lifetime


Compiler translates our codes into assembly
Compiler should/must know the size of spze in memory which each variable consumers ==> Compile time vs. Run Time


u8 = 양의 정수 
char는 u8로 casting될 수 있다.
char is unicode. 글자 하나하나는 그 글자를 표현하는 독립적인 숫자가 있다. 

cast

i32

std::mem::size_of::<char>()

std ==> Standard Libary
:: ==> 하위, 디렉토리 구조
mem ==> 파일, 모듈
:: ==> 
size_of ==> Function or Data Type or Constant
:: ==> 
<char> ==> <type> == Function's type
number ==> i32, i64, f32
() ==> Function call

pub const fn len(&self) -> usize
pub : Anyone can use this function in his/her code.
fn : Function
len : name
(&self) : argument, & ref, 
-> : return
usize : the type of the return value of function is usize