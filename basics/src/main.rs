fn main() {
    let greeting = "hello, world";
    let crash_reason = "I crashed";
    panic!("Reason {}", crash_reason);
    println!("The Greeting is {}", greeting);
}
