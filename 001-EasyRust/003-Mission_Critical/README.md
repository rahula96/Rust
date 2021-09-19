# [러스트] 003 - Mission Critical 언어란?

https://youtu.be/280bLoc73RQ

사용자 <==> 스마트폰 <=> 웹서버

사용자 <=> 스마트폰 == Front end language ==> Javascript/Typescript
스마트폰<=> 웹서버 or 웹서버 내부의 동작 ==> Back end ==> 나머지 모든 언어

Higher level = python, Ruby, R, Elixir
Lower level = C/C++, Java, Rust
C등의 언어 그 자체는 매우 정확 => Mission Critical ==> 오작동이 가능한 적도록 설계된 언어 => 사람(개발자)의 실수를 방지할 매커니즘은 없다 => 오작동의 가능성이 적지 않다.
Rust => Safety Fist 사람이 실수하는 것을 원천적으로 봉쇄 => 컴파일러가 경고 + 컴파일을 차단하도록 설계

Mission Critical 컴퓨터가 실수하거나 다운되거나 해서는 안되는 일

Rust의 용도는 사람의 실수를 제도적으로, 시스템적으로 방지하도록 만들어진 언어