fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);

    println!("{:?}", my_vec);
    println!("{:#?}", my_vec);

    println!("{}", my_vec[0]);

    let mut my_vec2 : Vec<String> = Vec::new();

    println!("{:?}", my_vec2);

    let mut my_vec3 : Vec<i32> = vec![1, 2, 3];

    println!("my_vec3: {:?}", my_vec3);
}