fn main() {
    // let mut num_vec = Vec::new();
    let mut num_vec = Vec::with_capacity(8);

    println!("num_vec.capacity(): {}", num_vec.capacity());
    num_vec.push('a');
    println!("num_vec.capacity(): {}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("num_vec.capacity(): {}", num_vec.capacity());
    num_vec.push('a');
    println!("num_vec.capacity(): {}", num_vec.capacity());
    println!("num_vec.len(): {}", num_vec.len());
}