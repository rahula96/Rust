enum ThingInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: i32) -> ThingInTheSky {
    match time {
        6..=18 => ThingInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingInTheSky::Stars(String::from("I can see the star")),
    }
}

fn check_skystate(state: &ThingInTheSky) {
    match state {
        ThingInTheSky::Sun(description) => println!("{}", description),
        ThingInTheSky::Stars(n) => println!("{}", n),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}