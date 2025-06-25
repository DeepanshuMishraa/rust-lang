fn main() {
    // Stack allocation - ownership is straightforward
    let x = 5; // x owns value 5
    let y = x; // Copy of value 5 is created, both x and y own their own 5
    println!("x: {}, y: {}", x, y); // Both still accessible

    // Heap allocation - ownership is moved
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // Ownership moves from s1 to s2
    // println!("{}", s1); // This would fail - s1 no longer owns the value
    println!("s2: {}", s2); // This works - s2 is the owner

    // References allow borrowing without taking ownership
    let s3 = String::from("world");
    let len = calculate_length(&s3); // Borrows s3 without taking ownership
    println!("Length of '{}' is {}", s3, len); // s3 still usable here

    // Mutable references
    let mut s4 = String::from("hello");
    change(&mut s4); // Mutable borrow
    println!("Changed string: {}", s4);

    // Cloning creates a deep copy, allowing multiple owned copies
    let original = String::from("clone me");
    let cloned = original.clone(); // Creates a new copy with its own ownership
    println!("Original: {}, Cloned: {}", original, cloned); // Both accessible

    // Clone is useful when you need to keep the original
    let numbers = vec![1, 2, 3];
    let numbers_copy = numbers.clone();
    println!("Original vec: {:?}", numbers);
    println!("Cloned vec: {:?}", numbers_copy);

    // Without clone, this would fail due to moved ownership
    let s5 = String::from("don't move me");
    process_string(s5.clone()); // Clone allows us to keep using s5
    println!("Original still here: {}", s5);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}

fn process_string(s: String) {
    println!("Processing string: {}", s);
}
