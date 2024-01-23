///  Enum to represent different geometric shapes
/// - Typical Use of Enums
/// - Associated Data
/// - Associated Methods
#[allow(dead_code)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

// Implementation of associated methods for the Shape enum
impl Shape {
    // Associated method to print the area of the shape
    fn print_area(&self) {
        match self {
            Shape::Circle(radius) => println!("Area of Circle: {:.2}", 3.14 * radius * radius),
            Shape::Rectangle(width, height) => println!("Area of Rectangle: {:.2}", width * height),
            Shape::Square(side) => println!("Area of Square: {:.2}", side * side),
        }
    }
}

// Enum to represent status
#[allow(dead_code)]
enum Status {
    Ok,
    NotFound,
    Error,
}

// Enum with methods
/// Enums for Modeling Events
/// Associated Data and Specific Behavior
#[allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    Click { x: i64, y: i64 },
}

impl WebEvent {
    fn print_info(&self) {
        match self {
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            WebEvent::Click { x, y } => println!("Clicked at ({}, {})", x, y),
        }
    }
}

pub fn default_function() {
    // Using the Shape enum and its associated methods
    let circle = Shape::Circle(2.5);
    circle.print_area();

    // Using the Status enum
    let _status = Status::Ok;

    // Using the WebEvent enum and its associated method
    let click_event = WebEvent::Click { x: 10, y: 20 };
    click_event.print_info();
}