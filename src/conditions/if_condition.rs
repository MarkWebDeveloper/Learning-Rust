pub fn make_decision() {
    let a = 10;
    let b = 20;
    let c;
    if a > b {
        c = a + b;
    } else if a < b {
        c = a - b;
    } else {
        c = a
    }
    println!("The IF condition result is: {0}", c);
    println!("--------------------------------------------------");
}