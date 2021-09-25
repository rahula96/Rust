#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn main() {
    use Season::*;
    let four_season = vec![Spring,Summer,Autumn, Winter];

    for season in four_season {
        // println!("{}", season as u32);
        println!("{:?}", season);
    }
}