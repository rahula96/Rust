enum VeryVerboseEnumOfThingsToDoWithNumber {
    Add,
    Substract,
}

impl VeryVerboseEnumOfThingsToDoWithNumber {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            self.Add => x + y,
            self.Substract => x - y,
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumber;

fn main() {
    let x = Operations::Add;
}