fn main() {
    let my_vec1: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into(); // default i32

    println!("my_vec1:{:?}", my_vec1);
    println!("my_vec2:{:?}", my_vec2);
}