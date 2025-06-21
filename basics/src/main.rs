fn main() {
    let greeting = "hello, world";
    let crash_reason = "I crashed";
    // panic!("Reason {}", crash_reason);

    let x = 1.1;
    let y = 2.2; // float 

    //mutable variable

    // let z = 1;
    // z = 2; // Error as z is not mutable

    let mut a = 1;
    a = 2; // No error as a is mutable

    // let mut  b:f64 = 1.1;
    // // b = 2; // Error as cannot assign int to a floa


    let one_thousand = 1_000;
    let exactly_three = 10/3;

    println!("one_thousand = {}", one_thousand);
    println!("exactly_three = {}", exactly_three);

    /*
    integer types in rust :
    - unsigned: u8, u16, u32, u64, u128, usize
    - signed: i8, i16, i32, i64, i128, isize
    - float: f32, f64
    - boolean: bool
    - char: char
    - tuple: (i32, i32, i32)

    signed-> only negative values
    unsigned-> only positive values

    usize-> size of the pointer
    isize-> size of the pointer

    f32-> 32 bit float
    
    */

    // true as u8 -> 1
    // false as u8 -> 0

    let is_adult:bool = true;

    let is_adult:bool = !is_adult;

    println!("is_adult = {}", is_adult);

    //conditionals
    if is_adult{
        println!("You are an adult");
    }else if is_adult==false{
        println!("You are not an adult");
    }else{
        println!("You are not an adult");
    }

    
    //ternary operator
    let is_adult = if is_adult{true}else{false};
    println!("is_adult = {}", is_adult);


    println!("x + y = {}", x + y);
    println!("The Greeting is {}", greeting);

    println!("x * y = {}", multiply_both(2.0,3.0));
}




fn multiply_both(x:f64,y:f64) -> f64{
   return x*y;
}

fn converting_to_something(x:i64,y:u8)->i64{
    // using the as keyword to convert the type of y to i64
    return x*(y as i64);
}
