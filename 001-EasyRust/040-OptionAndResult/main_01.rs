
fn take_fifth(value: Vec<i32>) -> i32 {
    value[4]
}

fn main() {
    let new_vec = vec![1, 2];
    let index = take_fifth(new_vec); // hread 'main' panicked at 'index out of bounds: the len is 2 but the index is 4', main_01.rs:3:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    println!("{:?}", index);
}