/// Traits é um recurso que permite definir um contrato para os métodos que um tipo pode implementar.
///Semelhantes a interfaces em outras linguagens de programação, mas têm algumas diferenças fundamentais.

// Defining a trait with associated methods
trait MathOperations {
    fn add(a: i32, b: i32) -> i32;
    fn subtract(a: i32, b: i32) -> i32;
    fn increment(&mut self, amount: i32);
    fn decrement(&mut self, amount: i32);
    fn new(value: i32) -> Self;
}

// Implementing
struct Calculator {
    value: i32,
}

impl MathOperations for Calculator {
    fn new(value: i32) -> Self {
        Calculator { value }
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    fn increment(&mut self, amount: i32) {
        self.value += amount;
    }

    fn decrement(&mut self, amount: i32) {
        self.value -= amount;
    }
}

pub fn default_function() {
    // Using "static"
    let result_addition = Calculator::add(5, 3);
    let result_subtraction = Calculator::subtract(8, 2);

    println!("Addition: {}", result_addition);
    println!("Subtraction: {}", result_subtraction);

    // Using "class""
    let mut calculator = Calculator::new(10);
    calculator.increment(10);
    calculator.decrement(4);

    println!("After Incrementing and Decrementing: {}", calculator.value);
}
