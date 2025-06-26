/*
Slices in Rust

Slices are a way to reference a contiguous sequence of elements in a collection

Slices are defined by a pointer to the first element and a length

Slices are useful when you want to refer to a part of a collection without copying the whole collection

Slices are defined by a pointer to the first element and a length

struct SliceMetadata{
first_element_index: usize,
length: usize,
}

*/

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; //[start..end]
    let world = &s[6..11]; //[start..end]
    println!("{} {}", hello, world);


    //slice doesnt own the elemenent just the reference to the elements

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}
