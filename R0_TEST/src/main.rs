// This code contains some basic examples of command line triggered math operations

use std::io;
fn main() {
    // Print to console with arguments
    println!("Hello, world! {1}, {0}", 3, 2);

    // Print console with a named tuple argument
    println!("{name:?}", name = (3, "hello"));
    println!("{decimal:X}", decimal = 255);

    // Read from console
    let mut requested_operation: String = String::new();
    io::stdin().read_line(&mut requested_operation).unwrap();
    print!("You requested: {}", requested_operation);
}
