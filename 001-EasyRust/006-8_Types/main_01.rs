
#[allow(unused_variables)] // attribute
fn main() {
    let first_letter = 'A'; // character is single quoted.
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    println!("{}", first_letter);
}