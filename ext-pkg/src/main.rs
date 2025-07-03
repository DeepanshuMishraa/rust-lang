use chrono::prelude::{Utc,Local};
//use chrono::prelude::* -> import everything from chrono
use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();
    let utc = Utc::now();
    println!("UTC time: {}", utc);

    let local = Local::now();
    println!("Local time: {}",local);

    // let var = env::var("TEST_VAR").unwrap(); //unwrap() will panic if the variable is not found. shouldnt use this in production as it will crash the program

    let var = env::var("TEST_VAR");

    match var{
        Ok(val)=>println!("TEST_VAR: {}", val),
        Err(_e)=>println!("TEST_VAR not found"),
    }

    // println!("TEST_VAR: {}", var);
}
