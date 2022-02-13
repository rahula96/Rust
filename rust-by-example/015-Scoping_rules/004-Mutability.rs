
// #[derive(Debug)]
#[derive(Debug, Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2021;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut  mutabook = immutabook;

    println!("immutabook: {:?}", immutabook);
    println!("mutabook: {:?}", mutabook);

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    // new_edition(&mut immutabook); cannot borrow as mutable

}