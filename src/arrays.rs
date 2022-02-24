// Arrays - fixed list where elements are the same type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // re-assign value
    numbers[0] = 8;

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
}