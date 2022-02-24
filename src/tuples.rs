// Group together values of different types;
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Charlie","UK",18);

    println!("{} is from the {} and is {} y/o", person.0,person.1,person.2)
}