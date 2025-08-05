use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User{
    username: String,
    email: String,
}


fn main() {
    let u = User{
        username: String::from("john_doe"),
        email: String::from("mail.com"),
    };

    let mut v:Vec<u8> = Vec::new();

    let ans = u.serialize(&mut v).unwrap();

    println!("Serialized User: {:?}", v);

    let user = User::try_from_slice(&v).unwrap();

    println!("deserialised user: {}",user.username);
}
