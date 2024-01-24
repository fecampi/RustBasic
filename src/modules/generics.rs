// Permite escrever código que pode ser parametrizado por diferentes tipos.
// Isso proporciona flexibilidade e reutilização de código para diferentes tipos de dados.

// Generic function that returns the larger of two values
fn get_larger<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

// Generic struct representing a pair of values
struct Pair<T, U> {
    first: T,
    second: U,
}

pub fn default_function() {
    // Example of the generic function
    let larger_number = get_larger(42, 17);
    let larger_character = get_larger('a', 'z');

    println!("Larger number: {}", larger_number);
    println!("Larger character: {}", larger_character);

    // Example of the generic struct
    let numeric_pair = Pair {
        first: 42,
        second: 3.14,
    };
    let textual_pair = Pair {
        first: "Hello",
        second: "World",
    };

    println!(
        "Numeric Pair: {} {}",
        numeric_pair.first, numeric_pair.second
    );
    println!(
        "Textual Pair: {} {}",
        textual_pair.first, textual_pair.second
    );
}
