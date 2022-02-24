
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of buts they take in memory. u = unsigned)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays (fixed length)
pub fn run() {
    const VERIFIED:bool = true;
    const CHARACTER:char = 'a';
    const FL:f32 = 1.12345;
    const NUM:i32 = 50;

    println!("Verified={verified}\nCharacter={character}\nFloat={fl}\nNum={num}",verified=VERIFIED,character=CHARACTER,fl=FL,num=NUM);

    // Find Max Size
    println!("Max I32: {}", std::i32::MAX);
    println!("Max I64: {}", std::i64::MAX);
}