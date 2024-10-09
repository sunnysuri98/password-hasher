use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand_core::OsRng;

use crate::error::*;

pub fn create_hashed_password(password: &str) -> Result<String, ErrorMessage> {
    if password.trim().is_empty() {
        return Err(ErrorMessage::EmptyPasswordError);
    } else if password.len() >= 64 {
        return Err(ErrorMessage::LengthError);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HashingError)?
        .to_string();

    Ok(hashed_password)
}


pub fn compare(password: &str,hashed_password:&str)->Result<bool,ErrorMessage>{
    if password.trim().is_empty() {
        return Err(ErrorMessage::EmptyPasswordError);
    } else if password.len() >= 64 {
        return Err(ErrorMessage::LengthError);
    }

    let parsed_hash= PasswordHash::new(hashed_password).expect("Unable to parsed hash");

    let password_matches= Argon2::default().verify_password(password.as_bytes(), &parsed_hash).map_or(false, |_| true);

    Ok(password_matches)




}
