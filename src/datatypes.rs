pub fn print_datatypes() {
    let integer: i32 = 10;
    let float: f64 = 10.0;
    let boolean: bool = true;
    let character: char = 'a';
    let string: &str = "Hello, World!";
    println!("Integer: {0}, Float: {1}, Boolean: {2}, Character: {3}, String: {4}", integer, float, boolean, character, string);
    println!("--------------------------------------------------");
}