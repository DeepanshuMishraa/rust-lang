use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct User{
    username:String,
    email:String,
}


fn main() {
    let u = User{
        username:String::from("john_doe"),
        email:String::from("Mynameis@mail.com")
    };

    let serialized_string = serde_json::to_string(&u);

    match serialized_string{
        Ok(s) =>{
            println!("Serialized User: {}",s);
        },
        Err(_)=>{
            println!("Error serializing user");
        }
    }


    let user_string = String::from("{\"username\": \"sam_smith\", \"email\": \"sam@mail.com\"}");

    let deserialized_user:Result<User,serde_json::Error> = serde_json::from_str(&user_string);

    match deserialized_user {
        Ok(user) => {
            println!("Deserialized User: {:?}", user);
        },
        Err(e) => {
            println!("Error deserializing user: {}", e);
        }
    }

}
