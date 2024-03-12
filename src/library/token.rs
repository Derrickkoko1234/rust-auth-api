// handles creation and validation of tokens
use std::{env, time::{Duration,SystemTime}};
use super::constant;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize,Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}

pub fn create_token(user_id:&str)->String{
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY not found");
    let duration = SystemTime::now() + Duration::from_secs(constant::THREE_DAYS_IN_SECONDS as u64); // 3 days
    let tk_claims = Claims {
        sub: user_id.to_owned(),
        exp: duration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
        iss: constant::APP_DOMAIN_NAME.to_owned(),
    };

    let token = encode(&Header::default(), &tk_claims, &EncodingKey::from_secret(jwt_key.as_ref()));
    match &token {
        Ok(tkn) => tkn.to_owned(),
        Err(_) => constant::EMPTY_STR.to_owned(),
    }
}

pub fn token_valid(token: &str)->(bool,String){
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY not found");
    let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
    let validation = Validation::new(Algorithm::HS256); // Adjust algorithm if needed
    let claims = decode::<Claims>(token, &decoding_key, &validation);
    match &claims {
        Ok(claim) => (claim.claims.sub.len()>0,claim.claims.sub.to_owned()),
        Err(_) => (false,constant::EMPTY_STR.to_owned()),
    }
}