pub fn run() {
    let age = 18;
    let check = false;

    // if/else
    if age >= 21 && check {
        println!("{} is greater than/equal to 21", age);
    }
    else if age < 21 && check {
        println!("{} is NOT great than/equal to 21", age);
    } else {
        println!("Check required");
    }

    // shorthand if
    let is_legal = if age >= 21 { true } else { false };
    println!("is legal: {}",is_legal)
}