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
