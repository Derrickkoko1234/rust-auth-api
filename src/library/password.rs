// handles hashing and verification of passwords

use std::env;
use argon2::{self, Config};

// hashes password
pub fn hash_password(password: &str) -> String{
    let config = Config::default();
    let salt = env::var("PWD_HASH_SALT").expect("Failed to fetch password hashing salt");
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    hash
}

// verify password hash
pub fn verify_password(hashed_password: &str, password: &str) -> bool{
    let matches = argon2::verify_encoded(hashed_password, password.as_bytes()).unwrap();
    matches
}