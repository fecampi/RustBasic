
// Função auxiliar para somar dois números
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn greet() -> String {
    String::from("Hello Node")
}


pub fn custom_greet(name: String, age: i32, height: f64) -> String {
    format!("Hello, {}! You are {} years old and {} meters tall.", name, age, height)
}

pub fn calculate_hypotenuse(a: f64, b: f64) -> f64 {
    (a*a + b*b).sqrt()
}

pub fn sum_array(array: Vec<i32>) -> i32 {
    array.iter().sum()
}


