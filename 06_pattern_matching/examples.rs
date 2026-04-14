// ============ CHAPTER 6: PATTERN MATCHING ============

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("=== BASIC MATCH ===\n");
    
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Other"),
    }
    println!();
    
    
    println!("=== MATCH EXPRESSION WITH RETURN ===\n");
    
    let x = 5;
    let message = match x {
        1 => "One",
        2 => "Two",
        3..=5 => "Three to Five",
        _ => "Other",
    };
    println!("Message: {}", message);
    println!();
    
    
    println!("=== ENUM WITH MATCH ===\n");
    
    let msg = Message::Write(String::from("Hello!"));
    
    match msg {
        Message::Quit => println!("Quit command"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
    println!();
    
    
    println!("=== TUPLE DESTRUCTURING ===\n");
    
    let tup = (0, 1, 2);
    match tup {
        (0, y, z) => println!("First is 0: y={}, z={}", y, z),
        (x, 0, z) => println!("Second is 0: x={}, z={}", x, z),
        (x, y, 0) => println!("Third is 0: x={}, y={}", x, y),
        _ => println!("None are zero"),
    }
    println!();
    
    
    println!("=== STRUCT DESTRUCTURING ===\n");
    
    let p = Point { x: 0, y: 7 };
    
    match p {
        Point { x, y: 0 } => println!("On x-axis at x={}", x),
        Point { x: 0, y } => println!("On y-axis at y={}", y),
        Point { x, y } => println!("Not on either axis: ({}, {})", x, y),
    }
    println!();
    
    
    println!("=== RANGE PATTERNS ===\n");
    
    let x = 5;
    match x {
        1..=3 => println!("One through three"),
        4..=6 => println!("Four through six"),
        7..=9 => println!("Seven through nine"),
        _ => println!("Something else"),
    }
    println!();
    
    
    println!("=== MULTIPLE PATTERNS ===\n");
    
    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 | 4 => println!("Three or four"),
        _ => println!("Something else"),
    }
    println!();
    
    
    println!("=== IGNORING VALUES ===\n");
    
    let (x, _, z) = (1, 2, 3);
    println!("x = {}, z = {}", x, z);
    println!();
    
    
    println!("=== IGNORE REMAINING WITH .. ===\n");
    
    let p = Point { x: 5, y: 10 };
    match p {
        Point { x, .. } => println!("Only care about x: {}", x),
    }
    println!();
    
    
    println!("=== MATCH GUARD ===\n");
    
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than 5: {}", x),
        Some(x) => println!("Greater than or equal to 5: {}", x),
        None => println!("None"),
    }
    println!();
    
    
    println!("=== COMPLEX GUARD ===\n");
    
    let (x, y) = (4, true);
    match (x, y) {
        (4..=6, _) if y => println!("x between 4-6 and y is true"),
        (_, true) => println!("y is true"),
        (4..=6, _) => println!("x between 4-6 (but y is false)"),
        _ => println!("Other"),
    }
    println!();
    
    
    println!("=== IF LET ===\n");
    
    let opt = Some(5);
    
    // if let is shorthand for single pattern match
    if let Some(x) = opt {
        println!("Got value: {}", x);
    } else {
        println!("Got None");
    }
    println!();
    
    
    println!("=== IF LET WITH ENUM ===\n");
    
    let msg = Message::Move { x: 10, y: 20 };
    
    if let Message::Move { x, y } = msg {
        println!("Moving to ({}, {})", x, y);
    } else {
        println!("Not a move message");
    }
    println!();
    
    
    println!("=== WHILE LET ===\n");
    
    let mut stack = vec![1, 2, 3];
    println!("Stack: {:?}", stack);
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    println!();
    
    
    println!("=== MATCH WITH RESULTS ===\n");
    
    let divide_result = divide(10, 2);
    match divide_result {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    let divide_error = divide(10, 0);
    match divide_error {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    
    println!("=== NESTED MATCHING ===\n");
    
    let data = Some((1, 2, 3));
    match data {
        Some((x, y, z)) if x + y + z > 4 => println!("Triple sum > 4: sum = {}", x + y + z),
        Some((x, y, z)) => println!("Triple sum <= 4: sum = {}", x + y + z),
        None => println!("No data"),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(a / b)
    }
}
