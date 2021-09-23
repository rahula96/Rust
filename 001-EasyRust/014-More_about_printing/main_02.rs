
fn main() {
    println!("{:X}", '행' as u32);
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);
    println!("{:?}", '행' as u32);
    println!("{:?}", 'H' as u32);
    println!("{:?}", '居' as u32);
    println!("{:?}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u
}