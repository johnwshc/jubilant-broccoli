// Rust Fundamentals - Following Coursera Course

fn main() {
    println!("=== Rust Fundamentals ===\n");

    // 1. Variables and Mutability
    variables_demo();

    // 2. Data Types
    data_types_demo();

    // 3. Functions
    functions_demo();

    // 4. Control Flow
    control_flow_demo();
}

// Variables and Mutability
fn variables_demo() {
    println!("1. Variables and Mutability:");

    let x = 5;
    println!("   Immutable variable x: {}", x);

    let mut y = 10;
    println!("   Mutable variable y: {}", y);
    y = 15;
    println!("   Modified mutable y: {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("   Constant MAX_POINTS: {}\n", MAX_POINTS);
}

// Data Types
fn data_types_demo() {
    println!("2. Data Types:");

    // Integers
    let integer: i32 = 42;
    println!("   Integer: {}", integer);

    // Floating point
    let float: f64 = 3.5;
    println!("   Float: {}", float);

    // Boolean
    let is_rust_fun: bool = true;
    println!("   Boolean: {}", is_rust_fun);

    // Character
    let letter: char = 'R';
    println!("   Character: {}", letter);

    // String
    let greeting = String::from("Hello, Rust!");
    println!("   String: {}\n", greeting);
}

// Functions
fn functions_demo() {
    println!("3. Functions:");

    let sum = add_numbers(5, 7);
    println!("   Sum of 5 and 7: {}", sum);

    let product = multiply(4, 6);
    println!("   Product of 4 and 6: {}\n", product);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Control Flow
fn control_flow_demo() {
    println!("4. Control Flow:");

    // If/else
    let number = 7;
    if number < 5 {
        println!("   Number is less than 5");
    } else if number == 5 {
        println!("   Number is exactly 5");
    } else {
        println!("   Number is greater than 5");
    }

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("   Loop result: {}", result);

    // While loop
    let mut count = 3;
    print!("   Countdown: ");
    while count > 0 {
        print!("{} ", count);
        count -= 1;
    }
    println!("Go!");

    // For loop
    print!("   For loop: ");
    for i in 1..4 {
        print!("{} ", i);
    }
    println!();
}
