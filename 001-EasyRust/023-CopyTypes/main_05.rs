fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

fn main() {
    let my_number;

    {
        let number = {
            67
        };

        my_number = loop_then_return(number);
    }

    println!("my_number: {}", my_number);
}