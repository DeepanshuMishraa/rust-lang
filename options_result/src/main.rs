


fn main() {
    let something = 10/0; 

    match something {
        Ok(something)=>{
            println!(something);
        },
        Err(e)=>{
            println!("An error occurred: {}", e);
        }
    }
}
