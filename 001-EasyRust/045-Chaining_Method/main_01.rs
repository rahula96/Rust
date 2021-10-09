fn main() {
    let mut new_vec = Vec::new();
    let mut count = 1;
    
    while count < 11 {
        new_vec.push(count);
        count += 1;
    }

    println!("{:?}", new_vec);
}