fn main() {
    let str_vec = vec!["one", "two", "three"];

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);

    // println!("{:?}", (_, _, variable));
    println!("{:?}", variable);
}