//enums=> Enums are a way to define a type that can be one of a set of values.

enum Color{
    Green,
    Blue,
    Red,
    Custom{
        r: u8,
        g: u8,
        b: u8,
    }, // Custom is a struct with three fields r, g, b.
    Custom2(u8, u8, u8), // Custom2 is a tuple with three fields r, g, b.

}

//Color is a type and Green, Blue, Red are the variants of the Color variants.


fn main(){
    let go = Color::Green;
    let stop = Color::Red;
    let maybe = Color::Blue;
    let custom = Color::Custom{r: 255, g: 0, b: 0};
    let custom2 = Color::Custom2(255, 0, 0);

    //pattern matching 

    match go{
        Color::Green => println!("Go"),
        Color::Blue => println!("Go"),
        Color::Red => println!("Go"),
        Color::Custom{r, g, b} => println!("Custom color: r={}, g={}, b={}", r, g, b),
        Color::Custom2(r, g, b) => println!("Custom2 color: r={}, g={}, b={}", r, g, b),
    }

    let color_str = match maybe{
        Color::Green => "Green",    
        Color::Blue => "Blue",
        Color::Red => "Red",
        Color::Custom{r, g, b} => "Custom color",
        Color::Custom2(r, g, b) => "Custom2 color",
        _ => "Unknown color", // _ is a wildcard and it will match any other variant. Similar to default in switch case.
    }; // you can also set a variable to the result of the match

    println!("Color: {}", color_str);

    let my_string = String::from("Hello");
    let last_char:Option<char> = my_string.pop();

    let success:Result<i32, String> = Ok(200);
    let failure:Result<i32, String> = Err("Failed to connect".to_string());

}

//Option<T> is a type that is used to represent a value that may be present or absent.
//Option<T> is defined as follows:
//enum Option<T> {
//    Some(T),
//    None,
//}

//Some(T) is a variant that represents a value that is present.


//Result<T, E> is a type that is used to represent a value that may be present or absent.
//Result<T, E> is defined as follows:
//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

//Ok(T) is a variant that represents a value that is present.
