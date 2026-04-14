// ============ CHAPTER 5: STRUCTS & ENUMS ============

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
}

impl User {
    fn new(username: String, email: String, age: u32) -> User {
        User { username, email, age }
    }
    
    fn info(&self) {
        println!("Username: {}, Email: {}, Age: {}", self.username, self.email, self.age);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

enum UserStatus {
    Active,
    Inactive,
    Banned { reason: String },
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    println!("=== STRUCTS ===\n");
    
    let user = User::new(
        String::from("rustlover"),
        String::from("rust@example.com"),
        25,
    );
    user.info();
    println!("{:?}\n", user);
    
    
    println!("=== MUTABLE STRUCT ===\n");
    
    let mut user2 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
    };
    user2.age = 31;
    user2.info();
    println!();
    
    
    println!("=== STRUCT WITH METHODS ===\n");
    
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle: {:?}", rect);
    println!("Area: {} sq units", rect.area());
    println!("Perimeter: {} units\n", rect.perimeter());
    
    
    println!("=== TUPLE STRUCTS ===\n");
    
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);
    println!("Color - R: {}, G: {}, B: {}\n", black.0, black.1, black.2);
    
    
    println!("=== ENUMS - BASIC ===\n");
    
    let status1 = UserStatus::Active;
    let status2 = UserStatus::Inactive;
    
    // Match enum to determine variant
    match status1 {
        UserStatus::Active => println!("User is active"),
        UserStatus::Inactive => println!("User is inactive"),
        UserStatus::Banned { reason } => println!("User banned: {}", reason),
    }
    println!();
    
    
    println!("=== ENUMS WITH ASSOCIATED DATA ===\n");
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello Rust!"));
    let msg4 = Message::ChangeColor(255, 128, 0);
    
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
    println!();
    
    
    println!("=== OPTION<T> ENUM ===\n");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let no_number: Option<i32> = None;
    
    // Extract value with match
    match some_number {
        Some(n) => println!("Value: {}", n),
        None => println!("No value"),
    }
    
    // Extract with if let
    if let Some(n) = some_string {
        println!("String value: {}", n);
    }
    
    // Using unwrap_or for default
    let val = no_number.unwrap_or(0);
    println!("No number, default: {}\n", val);
    
    
    println!("=== RESULT<T, E> ENUM ===\n");
    
    let result_ok: Result<i32, String> = Ok(42);
    let result_err: Result<i32, String> = Err(String::from("Something went wrong"));
    
    match result_ok {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    match result_err {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}\n", e),
    }
    
    
    println!("=== OPTION METHODS ===\n");
    
    let opt = Some(5);
    
    // is_some() and is_none()
    if opt.is_some() {
        println!("Option has a value");
    }
    
    // map() - transform value
    let doubled = opt.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);
    
    // filter()
    let even = opt.filter(|x| x % 2 == 0);
    println!("Even filter: {:?}\n", even);
    
    
    println!("=== RESULT METHODS ===\n");
    
    let divide_result: Result<i32, &str> = divide(10, 2);
    
    match divide_result {
        Ok(n) => println!("Division result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
    
    let divide_error: Result<i32, &str> = divide(10, 0);
    
    // Using match
    match divide_error {
        Ok(n) => println!("Division result: {}", n),
        Err(e) => println!("Error: {}\n", e),
    }
    
    
    println!("=== BANNED USER WITH REASON ===\n");
    
    let banned_user = UserStatus::Banned { 
        reason: String::from("Spam behavior") 
    };
    
    match banned_user {
        UserStatus::Banned { reason } => println!("User banned because: {}", reason),
        _ => println!("User has other status"),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero!")
    } else {
        Ok(a / b)
    }
}
