fn main() {
    // Example 1: Hello World
    println!("Hello, World!");
    println!("Welcome to Rust!");
    
    // Example 2: Printing with variables
    let name = "Rust";
    let version = "2024";
    println!("Programming with {} edition {}", name, version);
    
    // Example 3: Debug printing (using dbg! macro)
    let x = 42;
    dbg!(x);
    
    // Example 4: Multiple values
    println!("The answer is: {}", 42);
    println!("Pi is approximately: {:.2}", 3.14159);  // Format to 2 decimals
    
    // Example 5: Using named arguments
    println!("Name: {name}, Version: {version}");
}
