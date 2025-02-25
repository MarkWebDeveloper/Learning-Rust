// match is a type of switch in Rust

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
    let my_option: Option<&str> = None;

    match my_option {
        Some(value) => println!("My favorite girls are {}", value),
        None => println!("I don't need girlfriends")
    }

    println!("--------------------------------------------------");
}

pub fn match_result() {
    let my_result: Result<i32, &str> = Ok(100);

    match my_result {
        Ok(result) => println!("The server responded with the code {}", result),
        Err(error) => println!("Error occurred during the request. The error code is: {}", error)
    }

    println!("--------------------------------------------------");
}

pub fn match_if_let() {
    let my_option: Option<&str> = Some("beautiful day");

    if let Some(value) = my_option {
        println!("I like to go outside when it's a {}", value);
    } else {
        println!("I'm gonna stay at home");
    }

    println!("--------------------------------------------------");
}
