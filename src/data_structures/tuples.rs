pub fn create_and_print_tuple () {
    let person: (&str, i32, i32) = ("Mark", 30, 188);
    println!("The person's name is {}", person.0);
    let (name, age, height) = person;
    println!("The person's attributes are: Name: {}, Age: {}, Height: {}", name, age, height);
    println!("--------------------------------------------------");
}