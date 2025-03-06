pub fn print_a_struct() {
    struct Person {
        name: String,
        age: u8,
        height: u8
    }

    let mark = Person {
        name: String::from("Mark"),
        age: 30,
        height: 188
    };

    println!("The person's name is {}, age is {} and height is {}.", mark.name, mark.age, mark.height);
    println!("--------------------------------------------------");
}   