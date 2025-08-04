use chrono::prelude::*; // importing everything from the chrono crate
use std::env;

fn main() {
    let utc = Utc::now(); // getting the current UTC time
    println!("Current UTC time: {}", utc); // printing the current UTC time

    let local = Local::now(); // getting the current local time  
    println!("Current local time: {}", local); // printing the current local time
    
    let redis_url = env::var("REDIS_URL"); 

    match redis_url {
        Ok(url)=> {
            println!("Redis URL: {}", url); // printing the Redis URL if it exists
        },
        Err(_) => {
            println!("REDIS_URL environment variable is not set."); // printing a message if the Redis URL is not set
        }
    }
}
