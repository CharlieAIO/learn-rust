// Vectors - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // re-assign value
    numbers[0] = 8;

    // push to vector
    numbers.push(50);
    numbers.push(29);

    // remove last element from vectors
    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("First value: {}", numbers[0]);

    // get length
    println!("array length: {}",numbers.len());

    // get bytes of memory
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    // mem::size_of_val(&numbers) if use is used at top of file or std::mem::size_of_val(&numbers)

    // get slice
    let slice: &[i32] = &numbers[0..2]; //get first 2 values
    println!("slice: {:?}",slice);

    // loop through vector values;
    for x in numbers.iter() {
        println!("{}",x);
    }

    // loop and mutate
    for y in numbers.iter_mut() {
        *y *= 2; 
        // multiplies by 2
    }
    println!("{:?}", numbers);
}