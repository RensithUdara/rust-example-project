// ============ CHAPTER 3: FUNCTIONS ============

fn main() {
    println!("=== BASIC FUNCTIONS ===\n");
    
    // Call a simple function
    greet();
    
    
    println!("=== FUNCTIONS WITH PARAMETERS ===\n");
    
    // Single parameter
    print_number(42);
    
    // Multiple parameters
    add_numbers(5, 3);
    
    
    println!("=== RETURN VALUES ===\n");
    
    // Function returns a value
    let result = add_five(10);
    println!("10 + 5 = {}", result);
    
    // Another return value example
    let multiplied = multiply(4, 5);
    println!("4 * 5 = {}", multiplied);
    
    
    println!("=== EXPRESSIONS vs STATEMENTS ===\n");
    
    // Expression: no semicolon, returns value
    let y = {
        let x = 3;
        x + 1  // Expression - returns 4
    };
    println!("Expression result: {}", y);  // 4
    
    
    println!("=== MULTIPLE RETURN VALUES (TUPLES) ===\n");
    
    // Return multiple values using tuple
    let (doubled, squared) = calculate_values(5);
    println!("Doubled: {}, Squared: {}", doubled, squared);
    
    // Function returning 3 values
    let (a, b, c) = triple_operation(2);
    println!("Double: {}, Square: {}, Cube: {}", a, b, c);
    
    
    println!("=== EARLY RETURNS ===\n");
    
    let div1 = safe_divide(10, 2);
    println!("10 / 2 = {}", div1);
    
    let div2 = safe_divide(10, 0);
    println!("10 / 0 = {}", div2);  // Returns 0 (early return)
    
    
    println!("=== TEMPERATURE CONVERTER ===\n");
    
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{:.1}°C = {:.1}°F", celsius, fahrenheit);
    
    
    println!("=== FUNCTION POINTERS ===\n");
    
    let operation: fn(i32, i32) -> i32 = add_nums;
    println!("Using function pointer: {} + {} = {}", 5, 3, operation(5, 3));
    
    
    println!("=== CLOSURES (ANONYMOUS FUNCTIONS) ===\n");
    
    // Simple closure
    let double = |x| x * 2;
    println!("Double of 5: {}", double(5));
    
    // Closure capturing environment
    let multiplier = 3;
    let multiply_with_captured = |x| x * multiplier;
    println!("5 * {} = {}", multiplier, multiply_with_captured(5));
    
    // Closure with explicit types
    let add_explicit = |x: i32, y: i32| -> i32 {
        x + y
    };
    println!("Add closure: {} + {} = {}", 10, 20, add_explicit(10, 20));
    
    
    println!("=== FUNCTION RETURNING FUNCTION ===\n");
    
    let my_func = get_multiplier(2);
    println!("Result: {}", my_func(5));  // 5 * 2 = 10
}


// ============ SIMPLE FUNCTION ============

fn greet() {
    println!("Hello, Rust!");
}


// ============ FUNCTION WITH PARAMETERS ============

fn print_number(x: i32) {
    println!("The number is: {}", x);
}

fn add_numbers(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}


// ============ RETURN VALUES ============

fn add_five(x: i32) -> i32 {
    x + 5  // No semicolon! This is an expression
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}


// ============ MULTIPLE RETURN VALUES ============

fn calculate_values(x: i32) -> (i32, i32) {
    let doubled = x * 2;
    let squared = x * x;
    (doubled, squared)
}

fn triple_operation(x: i32) -> (i32, i32, i32) {
    (x * 2, x * x, x * x * x)
}


// ============ EARLY RETURNS ============

fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        println!("Cannot divide by zero!");
        return 0;  // Early return
    }
    a / b
}


// ============ REAL WORLD EXAMPLE: TEMPERATURE CONVERTER ============

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}


// ============ FUNCTION POINTERS ============

fn add_nums(a: i32, b: i32) -> i32 {
    a + b
}


// ============ HIGHER ORDER FUNCTION (Returns closure) ============

fn get_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}
