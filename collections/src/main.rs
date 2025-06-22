//Structs = custom data types
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let mut point: (i64, i64, i64) = (1, 2, 3); // tuple
    let x = point.0;
    let y = point.1;
    let z = point.2;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let (x, y, z) = point; // destructuring
    let (x, y, _) = point; // destructuring with ignore

    point.0 = 4;
    println!("point: {:?}", point);

    let mut point2 = (1, 2, 3);
    point2.0 = 4;
    println!("point2: {:?}", point2); // {:?} is used to print the tuple. it is known as debug print

    //Unit => it is a tuple with no elements
    let unit = ();

    //Arrays->fixed size, same type, contiguous memory

    let mut years: [i64; 3] = [2020, 2021, 2022]; // array of 3 elements

    let [first, second, third] = years; // destructuring
    println!("first: {}", first);
    println!("second: {}", second);
    println!("third: {}", third);

    let [first, second, _] = years; // destructuring with ignore

    years[0] = 2023;

    //iterating
    for year in years.iter() { 
        println!("years: {}", year);
    }
}

fn new_point(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}
