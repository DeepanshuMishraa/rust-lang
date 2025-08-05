
fn main() {
    let str1 = String::from("hello");
    let str2 = String::from("world");
    let ans = longest_str(&str1, &str2);
    println!("{}", ans);
}

fn longest_str(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
