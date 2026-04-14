// ============ CHAPTER 9: ITERATORS & CLOSURES ============

fn main() {
    println!("=== ITERATOR BASICS ===\n");
    
    let v = vec![1, 2, 3];
    
    // Create an iterator
    let iter = v.iter();
    
    // Consume with for loop
    for num in iter {
        println!("Number: {}", num);
    }
    println!();
    
    
    println!("=== ITER() VS INTO_ITER() VS ITER_MUT() ===\n");
    
    // iter() - borrow
    let v1 = vec![1, 2, 3];
    for num in v1.iter() {
        println!("iter: {}", num);
    }
    println!("v1 still valid: {:?}", v1);
    
    // into_iter() - consume
    let v2 = vec![1, 2, 3];
    for num in v2.into_iter() {
        println!("into_iter: {}", num);
    }
    // println!("v2 not valid");
    
    // iter_mut() - mutable borrow
    let mut v3 = vec![1, 2, 3];
    for num in v3.iter_mut() {
        *num *= 2;
    }
    println!("After iter_mut: {:?}\n", v3);
    
    
    println!("=== MAP() ADAPTER ===\n");
    
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);
    println!();
    
    
    println!("=== FILTER() ADAPTER ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();
    println!("Evens: {:?}", evens);
    println!();
    
    
    println!("=== CHAINING ADAPTERS ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = v.iter()
        .filter(|x| *x % 2 == 0)      // Keep even: 2, 4
        .map(|x| x * x)                // Square: 4, 16
        .collect();
    println!("Chain result: {:?}", result);
    println!();
    
    
    println!("=== TAKE() AND SKIP() ===\n");
    
    let v: Vec<i32> = (1..10).take(3).collect();
    println!("First 3: {:?}", v);
    
    let v: Vec<i32> = (1..10).skip(5).collect();
    println!("After skip 5: {:?}", v);
    println!();
    
    
    println!("=== CONSUMER: SUM() ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
    
    let sum_evens: i32 = v.iter()
        .filter(|x| *x % 2 == 0)
        .sum();
    println!("Sum of evens: {}", sum_evens);
    println!();
    
    
    println!("=== CONSUMER: COUNT() ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    let count = v.iter().count();
    println!("Count: {}", count);
    
    let even_count = v.iter()
        .filter(|x| *x % 2 == 0)
        .count();
    println!("Even count: {}", even_count);
    println!();
    
    
    println!("=== CONSUMER: FIND() ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    if let Some(found) = v.iter().find(|x| *x == 3) {
        println!("Found: {}", found);
    }
    
    if let Some(found) = v.iter().find(|x| *x > 10) {
        println!("Found: {}", found);
    } else {
        println!("Not found");
    }
    println!();
    
    
    println!("=== CONSUMER: ANY() AND ALL() ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    let has_even = v.iter().any(|x| *x % 2 == 0);
    println!("Has even: {}", has_even);
    
    let all_positive = v.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);
    
    let all_greater_than_3 = v.iter().all(|x| *x > 3);
    println!("All > 3: {}", all_greater_than_3);
    println!();
    
    
    println!("=== COMPLEX CHAIN ===\n");
    
    let result: i32 = (1..10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(2)
        .sum();
    println!("Complex chain result: {}", result);
    // Even: 2, 4, 6, 8
    // Squared: 4, 16, 36, 64
    // Take 2: 4, 16
    // Sum: 20
    println!();
    
    
    println!("=== CLOSURES BASICS ===\n");
    
    // Simple closure
    let add_one = |x| x + 1;
    println!("Closure result: {}", add_one(5));
    
    // Explicit types
    let multiply = |x: i32| -> i32 { x * 2 };
    println!("Multiply closure: {}", multiply(5));
    println!();
    
    
    println!("=== CLOSURE WITH MULTIPLE LINES ===\n");
    
    let process = |x| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        squared
    };
    println!("Process: {}", process(5));
    println!();
    
    
    println!("=== CAPTURING ENVIRONMENT ===\n");
    
    let y = 4;
    let add_y = |x| x + y;  // Captures y
    println!("Add y: {}", add_y(5));  // 9
    
    let multiplier = 10;
    let multiply_by = |x| x * multiplier;
    println!("Multiply by: {}", multiply_by(5));  // 50
    println!();
    
    
    println!("=== MUTABLE CAPTURE ===\n");
    
    let mut sum = 0;
    let add_to_sum = |x| {
        sum += x;  // Captures mut reference to sum
    };
    
    add_to_sum(5);
    add_to_sum(3);
    println!("Sum: {}", sum);  // 8
    println!();
    
    
    println!("=== MOVE CLOSURE ===\n");
    
    let s = String::from("hello");
    let take_ownership = move || println!("Moved: {}", s);
    take_ownership();
    // println!("{}", s);  // Error - s was moved
    println!();
    
    
    println!("=== CLOSURES IN HIGHER ORDER FUNCTIONS ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using a closure with filter
    let threshold = 3;
    let filtered: Vec<_> = numbers.iter()
        .filter(|x| *x > threshold)
        .copied()
        .collect();
    println!("Filtered (> {}): {:?}", threshold, filtered);
    
    // Using a closure with map
    let multiplied: Vec<_> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("Multiplied: {:?}", multiplied);
    println!();
    
    
    println!("=== PASSING CLOSURES TO FUNCTIONS ===\n");
    
    let result = apply_twice(5, |x| x * x);
    println!("Apply twice (x*x on 5): {}", result);
    
    let result2 = apply_twice(10, |x| x + 1);
    println!("Apply twice (x+1 on 10): {}", result2);
}

fn apply_twice<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}
