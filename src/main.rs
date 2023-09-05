use std::io;

/// our Operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// our powerful calculator function
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    let mut input = String::new();

    // let's prompt the user for the inputs
    println!("Let's do some operations!");
    
    // let's ask for the first value
    println!("Please insert first number...");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input. On Panic!");
    let v1: f64 = input.trim().parse().expect("Input not a number...");

    // let's ask for the operation to perform...
    println!("Now the operation to be performed...");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input. On Panic!");
    let op: char = input.trim().parse().expect("Input not a char value...");

    // let's ask for the second and last value
    println!("and finally the second number!");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input. On Panic!");
    let v2: f64 = input.trim().parse().expect("Input not a number...");

    // let's create a new Operation
    let o = match op {
        '+' => Operation::Add(v1, v2),
        '-' => Operation::Subtract(v1, v2),
        '/' => Operation::Multiply(v1, v2),
        '*' => Operation::Divide(v1, v2),
        _ => panic!("On the disco"),
    };

    // let's call our calculate function
    let res: f64 = calculate(o);
    println!("Thanks! The result is: {}", res);
}
