pub fn run() {
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());

    // Push Char
    // hello.push('w');

    // Push String
    hello.push_str("World");

    // Capacity In bytes
    println!("Capacity: {}",hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' ? {}",hello.contains("World"));

    // replace
    println!("Replace: {}\n",hello.replace("Hello","Goodbye"));



    //  Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}",word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b'); 

    // Assertion testing (only show error if fail)
    assert_eq!(2,s.len());

    println!("{}",s)
}