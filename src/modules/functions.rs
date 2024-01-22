/// Adds two integers.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The sum of `a` and `b`.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The product of `a` and `b`.
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers and returns the quotient and remainder.
///
/// # Arguments
///
/// * `dividend` - The number to be divided.
/// * `divisor` - The number by which `dividend` will be divided.
///
/// # Returns
///
/// A tuple containing the quotient and remainder.
fn division_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

/// Prints a message to the console.
///
/// # Arguments
///
/// * `message` - The message to be printed.
fn print_message(message: &str) {
    println!("{}", message);
}

/// Increments a mutable number by a specified amount.
///
/// # Arguments
///
/// * `number` - The number to be incremented (mutable).
/// * `amount` - The amount to be added to the number.
fn mutable_increment(number: &mut i32, amount: i32) {
    *number += amount;
}

pub fn default_function() {
    // Examples of function calls
    let result_addition = add(3, 4);
    println!("Result of addition: {}", result_addition);

    let result_multiplication = multiply(5, 6);
    println!("Result of multiplication: {}", result_multiplication);

    let (quotient, remainder) = division_with_remainder(10, 3);
    println!("Quotient: {}, Remainder: {}", quotient, remainder);

    print_message("Hello, Rust!");

    let mut number = 7;
    mutable_increment(&mut number, 3);
    println!("Number after increment: {}", number);

    // Closures:
    let soma_closure = |a, b| a + b;
    println!("A soma é: {}", soma_closure(3, 5));
}