pub fn run() {
    // Print to console
    println!("hello.");

    // Basic formatting
    println!("Number: {}/{}/{}", 20, 2, 2022);

    // Positional Arguments
    println!("The date is {0}/{1}/{2}", 20, 2, 2022);

    // Named arguments
    println!(
        "{name} likes to play {sport}",
        name = "Charlie",
        sport = "Tennis"
    );

    // Placeholder traits
    println!("Binary: {:b} \nHex; {:x} \nOctal: {:o}",10,10,10);

    // Placeholder for debug
    println!("{:?}", (12,true,"hello"));

    
}
