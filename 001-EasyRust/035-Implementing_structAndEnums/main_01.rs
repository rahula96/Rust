#[derive(Debug)]
enum AnimalType {
    Cat, // variant
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType, // field
}

impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("Changing anmial to dog!!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        println!("Changing anmial to cat!!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("The anmial is a Cat!"),
            AnimalType::Dog => println!("The anmial is a Dog!"),
        }
    }
}

fn main() {
    let mut new_animal = Animal::new();
    // let new_animal = Animal::new(); // cannot borrow as mutable

    new_animal.check_type();
    
    new_animal.change_to_dog();
    new_animal.check_type();

    new_animal.change_to_cat();
    new_animal.check_type();

}