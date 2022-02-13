
fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    println!("{} of 0b{:b} prople know binary, teh other half doesn't", 1, 0xff);

    println!("{number:>width$}", number=1, width=5);
    println!("{number:0>width$}", number=1, width=6);

    // println!("My name is {0}, {1} {0}", "Bond");
    /* error: invalid reference to positional argument 1 (there is 1 argument)
    --> 003-FormattedPrint.rs:14:31
    |
    14 |     println!("My name is {0}, {1} {0}", "Bond");
    |                               ^^^
    |
    = note: positional arguments are zero-based */
    println!("My name is {0}, {1} {0}.", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3)); // Structure` cannot be formatted with the default formatter
    println!("This struct `{:?}` won't print...", Structure(3));
    println!("This struct `{:#?}` won't print...", Structure(3));
}