#[allow(unused_variables)]

fn main() {
    let name = "서태지";
    let other_name = String::from("Adrian Fahrenheit Țepeș");

    let new_name = other_name; 

    println!("{}", name);
    println!("{:?}", name);
    // println!("{}", other_name); // error[E0382]: borrow of moved value: `other_name`
    // println!("{}", other_name);
    // println!("{:?}", other_name);
    println!("{:?}", new_name);
}