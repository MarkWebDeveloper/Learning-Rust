// pub fn run_inifinite_loop () {
//     loop {
//         println!("I am an infinite loop!");
//     }
// }

pub fn run_loop_and_break () {
    let mut counter: i32 = 0;
    loop {
        println!("I am an infinite loop!");
        counter += 1;
        if counter == 10 {
            println!("The loop is stopped.");
            println!("--------------------------------------------------");
            break;
        }
    }
}