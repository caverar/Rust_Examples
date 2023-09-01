// This code contains some basic examples of command line triggered math operations

use std::io;
fn main() {
    // Print to console with arguments
    // println!("Hello, world! {1}, {0}", 3, 2);

    // Print to console with a named tuple argument
    // println!("{name:?}", name = (3, "hello"));
    // Print to console with a named decimal argument in hex
    // println!("{decimal:X}", decimal = 255);

    // Mutable string
    // let mut requested_operation: String = String::new();

    // Read from console and store in requested_operation
    // io::stdin()
    //     .read_line(&mut requested_operation)
    //     .expect("Failed to read line");
    // println!("You requested: {}", requested_operation);

    // CLI Menu
    println!("Please select an operation: ");
    let mut requested_operation: String = String::new();
    io::stdin()
        .read_line(&mut requested_operation)
        .expect("Failed to read line");
    if requested_operation == "1" {
        println!("Operation 1");
    }
}
