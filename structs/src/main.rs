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


enum Shape{
    Cirlce(f32),
    Square(f32)
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

    let shape = Shape::Cirlce(5.0);
    match shape{
        Shape::Cirlce(radius) => println!("The radius of the circle is: {}", radius),
        Shape::Square(side) => println!("The side of the square is: {}", side),
    }

    let area = calculate_area(shape);
    println!("The area of the shape is: {}", area);
}


fn calculate_area(s:Shape)->f32{
    match s{
        Shape::Cirlce(radius)=> 3.14*radius*radius,
        Shape::Square(side)=> side*side,
    }
}
