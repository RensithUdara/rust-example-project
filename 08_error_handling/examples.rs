// ============ CHAPTER 8: ERROR HANDLING ============

use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeNumber,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero!"),
            MathError::NegativeNumber => write!(f, "Cannot use negative number!"),
        }
    }
}

impl std::error::Error for MathError {}

fn main() {
    println!("=== PANIC ===\n");
    
    // This would panic:
    // panic!("Something went wrong!");
    
    println!("Panics are for unrecoverable errors");
    println!();
    
    
    println!("=== RESULT ENUM BASICS ===\n");
    
    let result_ok: Result<i32, String> = Ok(42);
    let result_err: Result<i32, String> = Err(String::from("Something failed"));
    
    println!("Ok result: {:?}", result_ok);
    println!("Err result: {:?}", result_err);
    println!();
    
    
    println!("=== MATCH ON RESULT ===\n");
    
    let division = divide(10, 2);
    match division {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    let bad_division = divide(10, 0);
    match bad_division {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    
    println!("=== UNWRAP() ===\n");
    
    let result = divide(10, 2);
    // let value = result.unwrap();  // OK
    // println!("Unwrapped: {}", value);
    
    // This would panic:
    // let bad = divide(10, 0).unwrap();
    
    println!("unwrap() extracts Ok value or panics on Err");
    println!();
    
    
    println!("=== EXPECT() ===\n");
    
    let result = divide(10, 2);
    let value = result.expect("Division failed");
    println!("Expect result: {}", value);
    
    // With error would panic with message:
    // let bad = divide(10, 0).expect("Division by zero!");
    println!();
    
    
    println!("=== UNWRAP_OR() ===\n");
    
    let result1 = divide(10, 2);
    let val1 = result1.unwrap_or(0);
    println!("OK case: {}", val1);
    
    let result2 = divide(10, 0);
    let val2 = result2.unwrap_or(-1);
    println!("Err case (with default): {}\n", val2);
    
    
    println!("=== MAP() ===\n");
    
    let result = divide(10, 2);
    let doubled = result.map(|x| x * 2);
    println!("Mapped: {:?}", doubled);  // Ok(10)
    
    let bad = divide(10, 0);
    let bad_mapped = bad.map(|x| x * 2);
    println!("Bad mapped: {:?}\n", bad_mapped);  // Err unchanged
    
    
    println!("=== AND_THEN() ===\n");
    
    let result = divide(10, 2)
        .and_then(|half| divide(half, 2))
        .and_then(|quarter| divide(quarter, 2));
    
    println!("Chained result: {:?}", result);
    println!();
    
    
    println!("=== CUSTOM ERROR TYPE ===\n");
    
    let result1 = safe_divide(10, 2);
    match result1 {
        Ok(r) => println!("10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
    
    let result2 = safe_divide(10, 0);
    match result2 {
        Ok(r) => println!("10 / 0 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
    
    let result3 = safe_divide(-10, 2);
    match result3 {
        Ok(r) => println!("-10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    
    println!("=== QUESTION MARK OPERATOR ===\n");
    
    match process_with_question_mark(10, 2) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("Error: {}", e),
    }
    
    match process_with_question_mark(10, 0) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    
    println!("=== IS_OK() AND IS_ERR() ===\n");
    
    let result_ok = divide(10, 2);
    let result_err = divide(10, 0);
    
    println!("OK case is_ok: {}", result_ok.is_ok());
    println!("OK case is_err: {}", result_ok.is_err());
    
    println!("ERR case is_ok: {}", result_err.is_ok());
    println!("ERR case is_err: {}", result_err.is_err());
    println!();
    
    
    println!("=== OPTION vs RESULT ===\n");
    
    let option = Some(5);
    match option {
        Some(n) => println!("Option has: {}", n),
        None => println!("Option is None"),
    }
    
    let result = Ok::<i32, String>(5);
    match result {
        Ok(n) => println!("Result is Ok: {}", n),
        Err(e) => println!("Result is Err: {}", e),
    }
    println!();
    
    
    println!("=== CONVERTING OPTION TO RESULT ===\n");
    
    let option = Some(10);
    let result = option.ok_or("No value");
    println!("Converted: {:?}", result);
    
    let no_option: Option<i32> = None;
    let no_result = no_option.ok_or("No value");
    println!("Converted error: {:?}", no_result);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero!"))
    } else {
        Ok(a / b)
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else if a < 0 {
        Err(MathError::NegativeNumber)
    } else {
        Ok(a / b)
    }
}

fn process_with_question_mark(a: i32, b: i32) -> Result<i32, MathError> {
    let half = safe_divide(a, b)?;  // Returns error early if fails
    let quarter = safe_divide(half, 2)?;  // Returns error early if fails
    Ok(quarter)
}
