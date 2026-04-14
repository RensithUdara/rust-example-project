// ============ CHAPTER 4: OWNERSHIP & BORROWING ============

fn main() {
    println!("=== OWNERSHIP BASICS ===\n");
    
    // Stack types (Copy)
    let x = 5;
    let y = x;  // Both x and y are valid
    println!("x = {}, y = {}", x, y);
    
    // Heap types (Move)
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    let s2 = s1;  // Ownership moved!
    // println!("s1 = {}", s1);  // ❌ ERROR: s1 no longer owns it
    println!("s2 = {}\n", s2);
    
    
    println!("=== BORROWING (REFERENCES) ===\n");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1
    println!("'{}' has length {}", s1, len);
    println!("s1 still valid: {}\n", s1);
    
    
    println!("=== MUTABLE REFERENCES ===\n");
    
    let mut s = String::from("hello");
    println!("Before: {}", s);
    
    change_string(&mut s);  // Mutable borrow
    println!("After: {}\n", s);
    
    
    println!("=== MULTIPLE IMMUTABLE BORROWS (OK) ===\n");
    
    let s = String::from("hello world");
    let r1 = &s;  // Immutable borrow
    let r2 = &s;  // Immutable borrow (OK - multiple readers)
    let r3 = &s;  // Immutable borrow (OK)
    
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
    println!("r3 = {}\n", r3);
    
    
    println!("=== CONFLICTING REFERENCES EXAMPLE ===\n");
    
    // This example shows what the compiler prevents:
    let mut s = String::from("hello");
    let r1 = &s;      // Immutable borrow
    let r2 = &s;      // Immutable borrow
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
    // Immutable borrows end here
    
    let r3 = &mut s;  // Mutable borrow (OK - no more immutable)
    println!("r3 = {}\n", r3);
    
    
    println!("=== STRING SLICES ===\n");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    
    println!("hello = {}", hello);
    println!("world = {}", world);
    
    // Can also use ranges
    let first_five = &s[..5];   // Same as &s[0..5]
    let last_six = &s[5..];     // From index 5 to end
    let whole = &s[..];         // Entire string
    
    println!("first_five = {}", first_five);
    println!("last_six = {}", last_six);
    println!("whole = {}\n", whole);
    
    
    println!("=== ARRAY SLICES ===\n");
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}\n", slice);
    
    
    println!("=== FIRST WORD FUNCTION ===\n");
    
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("First word: {}\n", word);
    
    
    println!("=== OWNERSHIP TRANSFER FUNCTION ===\n");
    
    let s = String::from("hello");
    takes_ownership(s);  // s moves into function
    // println!("{}", s);  // ❌ ERROR: s was moved
    
    let x = 5;
    makes_copy(x);       // x is copied (not moved)
    println!("x = {} (still valid)\n", x);
    
    
    println!("=== RETURNING OWNERSHIP ===\n");
    
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    
    let s2 = takes_returns_ownership(s1);
    println!("s2 = {}\n", s2);
    
    
    println!("=== REFERENCES AS PARAMETERS ===\n");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' has length {}\n", s1, len);
    
    
    println!("=== MUTABLE REFERENCE EXAMPLE ===\n");
    
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("After modification: {}\n", s);
    
    
    println!("=== PRACTICAL: BORROWING IN LOOPS ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    for num in &numbers {  // Borrow for iteration
        println!("Number: {}", num);
    }
    
    // numbers still valid (we borrowed, didn't move)
    println!("numbers still valid: {:?}\n", numbers);
}


// ============ BASIC BORROWING EXAMPLE ============

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but it doesn't own the String, so nothing happens


// ============ MUTABLE REFERENCE EXAMPLE ============

fn change_string(s: &mut String) {
    s.push_str(" world");
}


// ============ STRING SLICES ============

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}


// ============ OWNERSHIP TRANSFER FUNCTIONS ============

fn takes_ownership(s: String) {
    println!("In takes_ownership: {}", s);
}  // s goes out of scope and is freed


fn makes_copy(x: i32) {
    println!("In makes_copy: {}", x);
}  // x goes out of scope, but is just i32 (Copy type), so nothing special


// ============ RETURNING OWNERSHIP ============

fn gives_ownership() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller
}

fn takes_returns_ownership(s: String) -> String {
    s  // Return the ownership to caller
}


// ============ APPEND FUNCTION ============

fn append_world(s: &mut String) {
    s.push_str(" world");
}
