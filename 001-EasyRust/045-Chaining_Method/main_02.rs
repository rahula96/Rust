fn main() {
    // let new_vec = (1..=10).collect::<Vec<i32>>();
    let new_vec: Vec<i32> = (1..=10).collect();

    println!("{:?}", new_vec);
}