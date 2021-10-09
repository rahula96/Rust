
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

trait Animal {
    fn bark(&self) {
        println!("Woof Woof");
    }

    fn run(&self) {
        println!("The dog is running");
    }
}

impl Animal for Dog {
    fn run(&self) {
        println!("{} is running!",self.name);
    }
}

impl Animal for Cat {
    fn run(&self) {
        println!("{} is running!",self.name);
    }
}

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };


    let pansuni = Cat { 
        name: "Pansuni".to_string(),
    };
    

    rover.bark();
    rover.run();

    pansuni.bark();
    pansuni.run();

}