// match is a type of switch in Rust

use std::collections::btree_map::Values;

pub fn match_working_age() {
    let age: i32 = 28;
    match age {
        1..=16 => println!("Too soon to work"),
        17..=65 => println!("Time to work, pal."),
        _ => println!("Relax and enjoy your infinite vacations!"),
    }
    println!("--------------------------------------------------");
}

// match with Enums

pub fn match_season() {
    enum Season {
        Spring,
        Summer,
        Autumn,
        Winter
    }

    let current_season: Season = Season::Winter;

    match current_season {
        Season::Winter => println!("It's Winter!"),
        Season::Spring => println!("It's Spring!"),
        Season::Summer => println!("It's Summer!"),
        Season::Autumn => println!("It's Autumn!")
    }

    println!("--------------------------------------------------");
}

// match with option

pub fn match_option () {
    let my_option: Option<&str> = Some("blonds");

    match my_option {
        Some(value) => println!("My favorite girls are {}", value),
        None => print!("I don't need girlfriends")
    }

    println!("--------------------------------------------------");
}
