fn main() {
    // ============ IMMUTABILITY vs MUTABILITY ============
    println!("=== IMMUTABILITY vs MUTABILITY ===\n");
    
    let x = 5;
    println!("Immutable x: {}", x);
    
    let mut y = 5;
    y = 10;
    println!("Mutable y changed to: {}\n", y);
    
    
    // ============ TYPE INFERENCE vs EXPLICIT ============
    println!("=== TYPE INFERENCE vs EXPLICIT ===\n");
    
    let inferred = 42;  // Rust infers this is i32
    let explicit: i32 = 42;
    let float_val: f64 = 3.14;
    
    println!("Inferred: {}", inferred);
    println!("Explicit: {}", explicit);
    println!("Float: {}\n", float_val);
    
    
    // ============ INTEGER TYPES ============
    println!("=== INTEGER TYPES ===\n");
    
    let small_int: i8 = 127;           // Small range
    let medium_int: i32 = 2_147_483_647; // Large range with underscores
    let big_int: i64 = 9_223_372_036_854_775_807; // Huge range
    let unsigned: u32 = 4_294_967_295;  // Only positive
    
    println!("Small (i8): {}", small_int);
    println!("Medium (i32): {}", medium_int);
    println!("Big (i64): {}", big_int);
    println!("Unsigned (u32): {}\n", unsigned);
    
    
    // ============ INTEGER LITERALS ============
    println!("=== INTEGER LITERALS ===\n");
    
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    
    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}\n", binary);
    
    
    // ============ FLOATING POINT ============
    println!("=== FLOATING POINT ===\n");
    
    let x = 2.0;      // Default f64
    let y: f32 = 3.0; // Explicit f32
    let z = 3.14159;
    
    println!("f64 (default): {}", x);
    println!("f32 (explicit): {}", y);
    println!("Precise f64: {:.5}\n", z);  // Format to 5 decimals
    
    
    // ============ BOOLEAN ============
    println!("=== BOOLEAN ===\n");
    
    let t = true;
    let f: bool = false;
    
    println!("True: {}", t);
    println!("False: {}\n", f);
    
    
    // ============ CHARACTER ============
    println!("=== CHARACTER ===\n");
    
    let c = 'z';
    let emoji = '😻';
    
    println!("Character: {}", c);
    println!("Emoji: {}\n", emoji);
    
    
    // ============ STRINGS ============
    println!("=== STRINGS ===\n");
    
    let immutable_str = "Hello";  // &str - immutable string slice
    let mut mutable_string = String::from("Hello");  // String - mutable
    mutable_string.push_str(" World");
    
    println!("String slice: {}", immutable_str);
    println!("String (growable): {}\n", mutable_string);
    
    
    // ============ TUPLES ============
    println!("=== TUPLES ===\n");
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    println!("Full tuple: {:?}", tup);  // :? for debug printing
    println!("First element: {}", tup.0);
    println!("Second element: {}", tup.1);
    println!("Third element: {}\n", tup.2);
    
    // Destructuring tuple
    let (x, y, z) = tup;
    println!("Destructured - x: {}, y: {}, z: {}\n", x, y, z);
    
    
    // ============ ARRAYS ============
    println!("=== ARRAYS ===\n");
    
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Last element: {}\n", arr[4]);
    
    // Array with explicit type and size
    let arr_explicit: [i32; 3] = [10, 20, 30];
    println!("Explicit array: {:?}\n", arr_explicit);
    
    // Array initialized with same value
    let arr_same = [3; 5];  // [3, 3, 3, 3, 3]
    println!("Same value array: {:?}\n", arr_same);
    
    
    // ============ SHADOWING ============
    println!("=== SHADOWING ===\n");
    
    let x = 5;
    println!("First x: {}", x);
    
    let x = x + 1;  // Shadow x
    println!("Shadowed x: {}", x);
    
    let x = x * 2;  // Shadow x again
    println!("Shadowed again: {}\n", x);
    
    // Shadowing allows type change!
    let spaces = "   ";
    println!("Spaces (string): '{}'", spaces);
    
    let spaces = spaces.len();  // Now it's a number
    println!("Spaces (number): {}\n", spaces);
    
    
    // ============ TYPE CASTING ============
    println!("=== TYPE CASTING ===\n");
    
    let x = 5i32;
    let y = x as i64;  // Cast i32 to i64
    println!("i32 to i64: {} -> {}", x, y);
    
    let x = 3.6f64;
    let y = x as i32;  // Cast f64 to i32 (truncates decimal)
    println!("f64 to i32: 3.6 -> {}\n", y);
    
    
    // ============ CONSTANT ============
    println!("=== CONSTANT ===\n");
    
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);
}
