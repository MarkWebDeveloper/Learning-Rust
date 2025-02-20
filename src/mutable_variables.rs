#[allow(unused_assignments)]
pub fn change_and_print_variables() {
    let mut name: &str = "Mark";
    let profession = "Programmer";

    name = "Pavel";

    println!("{0}, is a good {1}", name, profession);
    println!("--------------------------------------------------");
}