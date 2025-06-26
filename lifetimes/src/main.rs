/*
Lifetimes in rust

Lifetimes are the scope of a reference. basically the lifetime of a reference is the scope of the variable that the reference is pointing to.
It is a way to ensure that a reference is valid for the duration of the program.

// Lifetimes are a way to tell the Rust compiler how long references should be valid
// They help prevent dangling references (references to data that no longer exists)

// Example 1: Simple lifetime demonstration
// In this case, both references must live at least as long as the function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Example 2: Struct with references must specify lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str  // This reference must live as long as the struct
}

// Example 3: Lifetime elision
// Some common patterns don't need explicit lifetimes because Rust can figure them out
fn first_word(s: &str) -> &str {  // Rust knows the return reference is tied to input
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Key points about lifetimes:
// 1. They prevent dangling references
// 2. Most of the time, lifetimes are implicit
// 3. Sometimes we need to help Rust understand how long references should live
// 4. The 'static lifetime means the reference can live for the entire program

*/

fn main() {
    println!("Hello, world!");
}
