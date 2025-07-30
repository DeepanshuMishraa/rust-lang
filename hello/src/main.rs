use hello::greet;

fn main() {
    let mut bun:i32 = 2;
    let (bunnies,carrots) = (3,5);
    bun +=1;
    println!("There are {} bunnies and {} carrots.", bunnies, carrots);
    println!("There are {} buns.", bun);

    const MAX_BUNNIES: i32 = 10; // constants should be in uppercase snakecase;
    let meme = "Hello brother";
    let meme = "Bye brother"; // shadowing the previous variable with the same name , allowed in
                              // rust
    println!("{}", meme); // prints the last value of meme
    println!("The maximum number of bunnies is {}.", MAX_BUNNIES);  // constants are immutable
        
    greet()
}

