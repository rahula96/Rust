enum ThingInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: i32) -> ThingInTheSky {
    match time {
        6..=10 => ThingInTheSky::Sun,
        _ => ThingInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingInTheSky) {
    match state {
        ThingInTheSky::Sun => println!("I can see the sun!"),
        ThingInTheSky::Stars => println!("I can see the starS")
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}