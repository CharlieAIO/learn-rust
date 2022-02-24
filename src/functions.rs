pub fn run() {
    // greeting("Hello", "Charlie")

    // bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}",get_sum);

    // Closure
    let add_nums = |n1: i32, n2:i32| n1+n2;
    println!("Closure Sum: {}", add_nums(60,20));
}


fn greeting(greet: &str, name: &str) {
    println!("{} {}",greet,name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}