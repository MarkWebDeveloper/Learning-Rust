pub fn make_a_slice() {
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("The slice is: {:?}", slice);
    println!("--------------------------------------------------");
}   