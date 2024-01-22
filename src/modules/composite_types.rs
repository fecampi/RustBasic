pub fn default_function() {
    // Example 1: Tuples
    let tuple_example: (i32, f64, char) = (42, 3.14, 'a'); // Declaration of a tuple

    println!("Tuple: {:?}", tuple_example);

    // Example 2: Array with Different Types
    let array_i32: [i32; 3] = [1, 2, 3]; // Array of i32
    let array_f64: [f64; 4] = [1.1, 2.2, 3.3, 4.4]; // Array of f64
    let array_char: [char; 2] = ['a', 'b']; // Array of char

    println!("Array of i32: {:?}", array_i32);
    println!("Array of f64: {:?}", array_f64);
    println!("Array of char: {:?}", array_char);

    // Example 3: Vec with Different Types
    let vec_i32: Vec<i32> = vec![1, 2, 3]; // Vec of i32
    let vec_f64: Vec<f64> = vec![1.1, 2.2, 3.3, 4.4]; // Vec of f64
    let vec_char: Vec<char> = vec!['a', 'b']; // Vec of char
    let mixed_types: Vec<&dyn std::fmt::Debug> = vec![&1, &3.14, &'a', &"Hello"]; // Vec with different types

    println!("Vec of i32: {:?}", vec_i32);
    println!("Vec of f64: {:?}", vec_f64);
    println!("Vec of char: {:?}", vec_char);
    for value in &mixed_types {
        println!("{:?}", value);
    }

    // Example 4: Slices (also applicable to Vec)
    let slice_example: &[i32] = &vec_i32[1..3]; // Declaration of a slice from the Vec

    println!("Slice: {:?}", slice_example);

    // Example 5: Strings
    let string_example: String = String::from("Hello, Rust!"); // Declaration of a String

    println!("String: {}", string_example);
    // Example 6: Vec with Dynamic Size
    let mut dynamic_vec: Vec<i32> = vec![1, 2, 3];

    // Adding elements dynamically
    dynamic_vec.push(4);
    dynamic_vec.push(5);

    println!("Dynamic Vec: {:?}", dynamic_vec);

    // Additional Vec Operations
    println!("Size of Vec: {}", dynamic_vec.len());
    println!("Capacity of Vec: {}", dynamic_vec.capacity());

    // Sorting Vec
    dynamic_vec.sort();

    // Removing elements
    dynamic_vec.pop();
    println!("Dynamic Vec after pop: {:?}", dynamic_vec);

    // Checking element existence
    if dynamic_vec.contains(&3) {
        println!("Dynamic Vec contains the number 3.");
    }

    // Mapping elements
    let squared_numbers: Vec<i32> = dynamic_vec.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);
}
