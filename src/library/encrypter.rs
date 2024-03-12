// handles encryption and decryption of data

use std::env;

use magic_crypt::{new_magic_crypt, MagicCryptTrait,MagicCryptError};

pub fn encrypt_data(plaintext:&str)->String{
    let enc_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");

    let mc = new_magic_crypt!(enc_key, 256);

    let base64 = mc.encrypt_str_to_base64(plaintext);
    base64
}

pub fn decrypt_data(ciphertext:&str)->Result<String,MagicCryptError>{
    let enc_key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");
    let mc = new_magic_crypt!(enc_key, 256);
    let plaintext = mc.decrypt_base64_to_string(&ciphertext)?;
    Ok(plaintext)
}