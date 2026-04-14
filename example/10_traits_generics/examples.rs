// ============ CHAPTER 10: TRAITS & GENERICS ============

use std::fmt;

// ============ GENERICS ============

fn print_largest<T: PartialOrd + std::fmt::Display>(items: &[T]) -> &T {
    let mut largest = &items[0];
    
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// ============ GENERIC STRUCT ============

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============ GENERIC WITH MULTIPLE TYPES ============

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: fmt::Display, U: fmt::Display> Pair<T, U> {
    fn print_pair(&self) {
        println!("First: {}, Second: {}", self.first, self.second);
    }
}

// ============ TRAITS ============

trait Animal {
    fn species(&self) -> &str;
    
    fn sound(&self) -> &str {
        "Some sound"
    }
    
    fn describe(&self) {
        println!("{} says {}", self.species(), self.sound());
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn species(&self) -> &str {
        "Dog"
    }
    
    fn sound(&self) -> &str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn species(&self) -> &str {
        "Cat"
    }
    
    fn sound(&self) -> &str {
        "Meow!"
    }
}

// ============ CUSTOM DISPLAY TRAIT ============

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

// ============ DRAWABLE TRAIT ============

trait Drawable {
    fn draw(&self);
}

impl Drawable for Point<f32> {
    fn draw(&self) {
        println!("Point at ({}, {})", self.x, self.y);
    }
}

// ============ CLONE EXAMPLE ============

#[derive(Clone, Debug)]
struct Employee {
    name: String,
    id: u32,
}

// ============ FROM TRAIT ============

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts: Vec<&str> = s.split(',').collect();
        Person {
            name: parts[0].to_string(),
            age: parts[1].parse().unwrap_or(0),
        }
    }
}

fn main() {
    println!("=== GENERIC FUNCTIONS ===\n");
    
    let numbers = vec![40, 50, 30, 10, 20];
    println!("Largest number: {}", print_largest(&numbers));
    
    let strings = vec!["Alice", "Bob", "Charlie"];
    println!("Largest string: {}\n", print_largest(&strings));
    
    
    println!("=== GENERIC STRUCTS ===\n");
    
    let int_point = Point { x: 5, y: 10 };
    println!("Int point: {:?}", int_point);
    println!("X: {}", int_point.x());
    
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Float point: {:?}", float_point);
    println!("Distance from origin: {}\n", float_point.distance_from_origin());
    
    
    println!("=== GENERIC WITH MULTIPLE TYPES ===\n");
    
    let pair = Pair {
        first: 5,
        second: "hello",
    };
    println!("Pair: {:?}\n", pair);
    
    let pair2 = Pair {
        first: 3.14,
        second: 42,
    };
    println!("Pair2: {:?}\n", pair2);
    
    
    println!("=== IMPLEMENTING TRAITS ===\n");
    
    let dog = Dog {
        name: "Buddy".to_string(),
    };
    dog.describe();
    
    let cat = Cat {
        name: "Whiskers".to_string(),
    };
    cat.describe();
    println!();
    
    
    println!("=== CUSTOM DISPLAY ===\n");
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Display: {}", person);
    println!("Debug: {:?}\n", person);
    
    
    println!("=== DRAWABLE TRAIT ===\n");
    
    let point = Point { x: 5.0, y: 10.0 };
    point.draw();
    println!();
    
    
    println!("=== CLONE TRAIT ===\n");
    
    let emp1 = Employee {
        name: "Alice".to_string(),
        id: 1,
    };
    
    let emp2 = emp1.clone();
    println!("Employee 1: {:?}", emp1);
    println!("Employee 2: {:?}", emp2);
    println!();
    
    
    println!("=== FROM TRAIT ===\n");
    
    let person: Person = "Bob,25".into();
    println!("Created from string: {}", person);
    println!();
    
    
    println!("=== TRAIT BOUNDS WITH WHERE ===\n");
    
    let data: Pair<i32, &str> = Pair {
        first: 42,
        second: "answer",
    };
    data.print_pair();
    println!();
    
    
    println!("=== TRAIT OBJECTS ===\n");
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "Rex".to_string(),
        }),
        Box::new(Cat {
            name: "Mittens".to_string(),
        }),
        Box::new(Dog {
            name: "Max".to_string(),
        }),
    ];
    
    for animal in animals {
        animal.describe();
    }
    println!();
    
    
    println!("=== GENERIC FUNCTION WITH TRAIT BOUND ===\n");
    
    let nums = vec![1, 2, 3, 4, 5];
    let strings = vec!["a", "b", "c"];
    
    print_vec(&nums);
    print_vec(&strings);
}

fn print_vec<T: fmt::Display>(items: &[T]) {
    print!("Items: ");
    for item in items {
        print!("{} ", item);
    }
    println!();
}
