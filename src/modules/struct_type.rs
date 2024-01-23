// Definition of a struct named Point
struct Point {
    x: f64,
    y: f64,
}

// Implementation of methods associated with the Point struct
impl Point {
    // Associated function to create a new Point instance
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Method that calculates the distance to the origin (0,0)
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn default_function() {
    // Creating an instance of the Point struct using the associated function
    let point = Point::new(3.0, 4.0);

    // Accessing the fields of the Point struct
    println!("Value of x: {}", point.x);
    println!("Value of y: {}", point.y);

    // Using the associated method to calculate the distance to the origin
    let distance = point.distance_to_origin();
    println!("Distance to the origin: {}", distance);
}
