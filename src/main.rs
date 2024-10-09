mod error;
mod hasher;
use hasher::{compare, create_hashed_password};

fn main() {
    let password = String::from("P@ssword123");

    match create_hashed_password(&password) {
        Ok(hashed_password) => match compare(&password, &hashed_password) {
            Ok(_)=>println!("Password matches!!"),
            Err(e)=> eprintln!("Error Comparing password: {}",e)
        },
        Err(e) => eprintln!("Error during creating hashed password: {}", e),
    }
}


