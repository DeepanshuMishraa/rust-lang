use std::ops::Add;

//Generics= > A generic in rust is a way to write code that can work with any data type.It allows
//you to define functions, structs, enums, and traits that can operate on multiple types without
//sacrificing type safety.


//Trait bound generics = > A trait bound is a way to specify that a generic type must implement a
//certain trait. for eg: + operator cannot be used on all types, it can only be used on types that
//can be added together, like integers and floats. So, if you want to write a function that adds
//two numbers together, you can use a trait bound to specify that the generic type must implement
//so that the + operator can be used on it.

struct User{
    name:String,
}

struct Rect<T>{
    width:T,
    height:T,
}

impl<T:std::ops::Mul<Output=T> + Copy> Rect<T>{
    fn area(&self)-> T{
        return self.width * self.height; // calculating the area of the rectangle
    }
}

fn main() {

    let u = User{
        name:String::from("John Doe"),
    };
    println!("Hello, world!");
    let sum = sum(5, 10); // using the sum function with integers
    println!("Sum of integers: {}", sum);
    print_anything("Hello, Rust!"); // using the print_anything function with a string 
    print_anything(42); // using the print_anything function with an integer 

    let rect = Rect{
        width: 5.0,
        height: 10.0,
    };

    println!("Area of rectangle: {}", rect.area()); // calculating and printing the area of the
}



/*
without the trait bound so this code will not compile 
fn sum<T>(a:T,b:T)->T{
    return a+b;
}

*/


//using trait bound generics to specify that T must implement the Add trait
fn sum<T:Add<Output=T>>(a:T,b:T)->T{
    return a + b; // using the + operator to add two values of type T
}


fn print_anything<T:std::fmt::Display>(value:T){
    println!("{}",value)
}
