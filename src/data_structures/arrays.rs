pub fn loop_through_array () {
    let days_of_the_week = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    for day in 0..3 {
        println!("The day is: {0}", days_of_the_week[day]);
    }
    println!("--------------------------------------------------");
    for day in days_of_the_week.iter() {
        println!("The day is: {0}", day);
    }
    println!("--------------------------------------------------");
}