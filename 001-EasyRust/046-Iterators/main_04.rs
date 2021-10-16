#[derive(Debug, Clone)]
struct Library {
    library_type : LibraryType,
    book: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.book.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            book: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.book.pop() {
            Some(book) => Some(book + " is found!"),
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("sjfaldjflasfkjasfl");

    for elem in my_library {
        println!("{}", elem);
    }
}