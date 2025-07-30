struct Rect{
    height:f32,
    width:f32,
}


enum Direction {
    North,
    South,
    East,
    West
}

impl Rect{
    fn area(&self)->f32{
        return self.height * self.width;
    } // member function


    fn print_something(){
        println!("This is a rectangle");
    } // static method. This cannot be called on an instance of the struct,
      // it can only be called on the struct itself.
}

fn main() {
    let r = Rect{

    height:10.0,
    width:20.0,
    };

    println!("The height {} and width {} are: ",r.height, r.width);

    let area = r.area();

    println!("The area of the rectangle is: {}", area);

    Rect::print_something(); // calling static method
                            
    let direction = Direction::North; // enum instance

    match direction {
        Direction::East => println!("The direction is East"),
        Direction::West => println!("The direction is West"),
        Direction::South => println!("The direction is South"),
        Direction::North => println!("The direction is North"),
    }
}
