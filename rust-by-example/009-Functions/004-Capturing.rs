
fn main() {
    // use std::mem;

    let color = String::from("green");

    let print = || println!("'color': {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    // print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    
    // let _reborrow = &count; 

    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("'movable': {:?}", movable);
        std::mem::drop(movable);
    };

    consume();

    let haystack = vec![1, 2, 3];
    
    let contains = move |needle| haystack.contains(needle);
    
    // println!("There're {} elements in vec", haystack.len());

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}