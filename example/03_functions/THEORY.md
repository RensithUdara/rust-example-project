# Chapter 3: Functions 🔧

## Function Basics

Functions are reusable blocks of code. In Rust, function names use `snake_case`.

```rust
fn main() {
    println!("In main!");
    
    another_function();
}

fn another_function() {
    println!("In another function!");
}
```

**Important:** You can define functions above or below `main()` - declaration order doesn't matter!

## Parameters

Functions can accept values called **parameters**:

```rust
fn main() {
    print_number(5);
    add_numbers(3, 4);
}

fn print_number(x: i32) {
    println!("The number is: {}", x);
}

fn add_numbers(x: i32, y: i32) {
    println!("Sum: {}", x + y);
}
```

**Type annotations are required** - Rust won't infer parameter types.

## Return Values

Functions can return values using `->` syntax:

```rust
fn main() {
    let x = add_five(5);
    println!("Result: {}", x);  // 10
}

fn add_five(x: i32) -> i32 {
    x + 5  // No semicolon! This is an expression, not a statement
}
```

### Statements vs Expressions

**Statements:** Instructions that end with `;`, return nothing
```rust
let x = 5;
let y = (let z = 6);  // ❌ ERROR - can't do this
```

**Expressions:** Return a value, **no semicolon!**
```rust
{
    let x = 3;
    x + 1      // ✅ Returns 4, no semicolon
}
```

The difference is crucial:
```rust
fn example_statement() {
    let x = 5 + 6;  // ✅ Statement - has semicolon
}

fn example_expression() -> i32 {
    5 + 6           // ✅ Expression - no semicolon!
}
```

### Common Mistake

```rust
fn wrong_return() -> i32 {
    5;  // ❌ ERROR! This is a statement (returns nothing)
}

fn correct_return() -> i32 {
    5   // ✅ Correct! This is an expression (returns 5)
}
```

## Multiple Return Values

### Using Tuples

```rust
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let (x, y) = swap(5, 10);
    println!("x: {}, y: {}", x, y);  // x: 10, y: 5
}
```

### Using Tuple Destructuring

```rust
fn calculate(x: i32) -> (i32, i32, i32) {
    let double = x * 2;
    let square = x * x;
    let cube = x * x * x;
    
    (double, square, cube)
}

fn main() {
    let (d, s, c) = calculate(3);
    println!("Double: {}, Square: {}, Cube: {}", d, s, c);
    // Double: 6, Square: 9, Cube: 27
}
```

## Early Returns

Return from a function before the end:

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0;  // Early return
    }
    
    a / b
}
```

## Function as Expressions

Functions can be assigned to variables (somewhat):

```rust
fn main() {
    let result = {
        let x = 3;
        x + 1
    };
    
    println!("Result: {}", result);  // 4
}
```

## Parameters with Default Values?

Rust **doesn't have default parameters** like many languages. You either:

1. Create multiple functions:
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn greet_default() {
    greet("Friend");
}
```

2. Use `Option` type (covered later):
```rust
fn hello(name: Option<&str>) {
    let name = name.unwrap_or("Friend");
    println!("Hello, {}!", name);
}
```

## Function Pointers

Store a function reference:

```rust
fn main() {
    let func: fn(i32) -> i32 = add_five;
    
    let result = func(10);
    println!("Result: {}", result);  // 15
}

fn add_five(x: i32) -> i32 {
    x + 5
}
```

## Closures (Anonymous Functions)

Functions without a name:

```rust
fn main() {
    let add_one = |x| x + 1;  // Closure
    
    println!("{}", add_one(5));  // 6
}
```

Closures capture their environment:

```rust
fn main() {
    let y = 4;
    let add_y = |x| x + y;  // Captures y!
    
    println!("{}", add_y(5));  // 9
}
```

## Diverging Functions

Functions that never return (rare):

```rust
fn panic_function() -> ! {
    panic!("This function never returns!");
}

fn infinite_loop() -> ! {
    loop {
        println!("Forever!");
    }
}
```

The `!` type is called the "never type".

## Summary

| Concept | Example |
|---------|---------|
| Basic function | `fn name() { }` |
| With parameters | `fn add(x: i32, y: i32) { }` |
| Return type | `fn add() -> i32 { 5 }` |
| Expression (no `;`) | `5 + 6` returns value |
| Statement (with `;`) | `let x = 5;` returns nothing |
| Multiple returns | `-> (i32, i32)` with tuple |
| Early return | `return value;` |
| Closure | `\|x\| x + 1` |

## Key Rules

✅ **DO:**
- Use snake_case for function names
- Specify parameter types explicitly
- Specify return types explicitly
- Use expressions instead of statements when returning

❌ **DON'T:**
- Forget type annotations on parameters
- Add `;` when you want to return a value
- Use variables with similar names in same scope
