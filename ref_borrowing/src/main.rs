//refrences in rust = pointers to a value. They are immutable by default and cannot be changed to point to a different value.

/*
// Borrowing in Rust
// Borrowing is the mechanism that allows you to reference data without taking ownership
// It's like lending something - the owner temporarily lets someone else use it

// Borrowing Rules in Rust:
// 1. At any given time, you can have either:
//    - One mutable reference (&mut T)
//    - Any number of immutable references (&T)
// 2. References must always be valid (no dangling references)
// 3. References cannot outlive the data they refer to

// The Borrow Checker:
// - A compile-time mechanism that enforces borrowing rules
// - Ensures memory safety without garbage collection
// - Prevents data races and invalid memory access

// Example demonstrating borrow checker rules:
let mut value = String::from("hello");

// Multiple immutable borrows are allowed
let ref1 = &value;
let ref2 = &value;
println!("Immutable refs: {} {}", ref1, ref2);

// After immutable refs are done, we can have one mutable borrow
let mut_ref = &mut value;
mut_ref.push_str(" world");

// This would fail compilation - can't mix mutable and immutable borrows:
// let another_ref = &value;        // Error: cannot borrow `value` as immutable
// println!("{} {}", mut_ref, another_ref);

// Borrow checker also prevents dangling references:
// let dangling_ref = {
//     let temp = String::from("temp");
//     &temp  // Error: `temp` does not live long enough
// };  // `temp` is dropped here while still borrowed


// Example 1: Immutable Borrowing
let original = String::from("hello");
let borrowed = &original;  // Borrow original immutably
println!("Original: {}, Borrowed: {}", original, borrowed);  // Both are accessible
// borrowed can read but cannot modify original

// Example 2: Mutable Borrowing
let mut value = String::from("hello");
let mut_borrow = &mut value;  // Borrow value mutably
mut_borrow.push_str(" world");  // Can modify through mutable borrow
println!("Modified value: {}", value);

// Example 3: Multiple Borrows
let data = String::from("multiple borrows");
let borrow1 = &data;
let borrow2 = &data;
println!("Can have multiple immutable borrows: {} {}", borrow1, borrow2);

// Example 4: Borrowing Rules in Practice
let mut text = String::from("example");
let read1 = &text;     // First immutable borrow
let read2 = &text;     // Second immutable borrow - OK
// let write1 = &mut text;  // This would fail - can't have mutable and immutable borrows
println!("Reading borrowed values: {} {}", read1, read2);

// After all immutable borrows are used, we can now borrow mutably
let write1 = &mut text;
write1.push_str(" modified");



*/

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //&s1 => borrow s1 and give the pointer to the function. it doesnt take the ownership of s1.
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed string: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
