pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        print!(" {} ",count);

        if count == 20 { break }
    }

    // while loop
    while count <= 100 {
        if count % 15  == 0 {
            println!("FIZZ BUZZ {}",count);
        }
        else if count % 3 == 0 {
            println!("FIZZ {}",count);
        }
        else if count % 5 == 0 {
            println!("BUZZ {}",count);
        } else {
            println!("{}",count)
        }
        count += 1;
    }

    // For range
    for x in 0..100 {
        if x % 15  == 0 {
            println!("FIZZ BUZZ {}",x);
        }
        else if x % 3 == 0 {
            println!("FIZZ {}",x);
        }
        else if x % 5 == 0 {
            println!("BUZZ {}",x);
        } else {
            println!("{}",x)
        }
    }
}