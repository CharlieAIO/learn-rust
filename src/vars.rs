
pub fn run() {
    //mut = mutatable (value can be changed)
    let mut day = "Sunday";
    println!("Today is: {}", day);
    day = "Friday";
    println!("Today is: {}", day);

    // Define constant
    const ID: i32 = 0043;
    println!("ID IS {}", ID);

    // Assign multiple variables
    let ( my_name, my_age ) = ("Charlie", 18);

    println!("My name is {} and my age is {}",my_name,my_age)
}