fn main() {
    let str = String::from("Hello,world");
    let len  = get_length(&str);

    //immutable reference  

    let s1 = String::from("Deepanshu");
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;

    println!("The string is: {}", s1);
    println!("The string is: {}", s2);
    println!("The string is: {}", s3);
    println!("The string is: {}", s4);

    //mutable references

    let mut st = String::from("Hello,world");
    let st1 = &mut st;
    //let st2 = &mut st; // This line would cause a compile-time error because you cannot have two
    //mutable references to the same data at the same time.

    println!("The length of the string is: {}", len);

    let stri = String::from("Hello,world");

    println!("The original String is : {}",stri);

    let len = calcualte_length(&stri);
    println!("The length of the string is: {}", len);
}


fn get_length(s:&String)->usize {
  return s.len();
}

// There can only be one mutable reference to a piece of data in a particular scope.


// There can be multiple immutable references to a piece of data in a particular scope.


fn calcualte_length(s:&String) -> usize{
    return s.len();
}
