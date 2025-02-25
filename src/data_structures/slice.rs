pub fn make_a_slice() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("The slice is: {:?}", slice);
    println!("--------------------------------------------------");
}   