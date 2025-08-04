// traits => Like interfaces in typescript, traits in Rust allow you to define shared behavior that
// can be implemented by different types. Traits can define methods that types must implement, and 
// // they can also define default implementations for those methods. This allows you to write code
// that can work with any type that implements a specific trait, without knowing the exact type at
// compile time.


struct Rect{
    width:f32,
    height:f32,
}

impl Shape for  Rect{
    fn area(&self)->f32 {
        return self.width * self.height
    }
}

struct Circle{
    radius:f32, 
}

impl Shape for  Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

trait Shape{
    fn area(&self) -> f32;
}


fn print_area_of_shape<T:Shape>(shape:T){
    println!("Area of shape: {}", shape.area());
}

fn main() {
    println!("Hello, world!");
    let rect = Rect {
        width: 5.0,
        height: 10.0,
    };

    let circle = Circle {
        radius: 3.0,
    };

    print_area_of_shape(rect); // printing the area of the rectangle
    print_area_of_shape(circle); // printing the area of the circle 
}
