enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star"),
            size if size >= 80 => println!("This is a good-sized star"),
            _ => println!("That star is pretty big"),
        }
    }

    println!("What about DeadStar? It's the number {}", DeadStar as u32);
}