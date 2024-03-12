use actix_web::error::{self,Error};
use rand::distributions::{Distribution, Uniform};
use redis::Client;
use crate::app::redis::Redis;

use super::constant;

// generates random nuber of a certain length
pub fn generate_random_code(len: usize)->String{
    let step = Uniform::new(constant::CODE_MIN, constant::CODE_MAX);
    let mut rng = rand::thread_rng();
    let choices: Vec<_> = step.sample_iter(&mut rng).take(len).collect();
    let code = choices
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(constant::EMPTY_STR);
    code
}

// verify supplied code
pub async fn verify_code(key: &str,content: &str,redis_client: &mut Client)->Result<bool,Error>{
    let k = format!("{}_{}",constant::CODE,key);
    let r = Redis::get_string_item(redis_client, &k);
    if r.to_owned().eq_ignore_ascii_case(content){
        Redis::delete_item(redis_client, &k);
        Ok(true)
    }else{
        Err(error::ErrorInternalServerError(mongodb::error::Error::custom("Invalid verification code")))
    }
}

pub fn generate_username(name: &str)->String{
    let name_split: Vec<&str> = name.split(" ").collect();
    format!("{}_{}",name_split[0],generate_random_code(constant::CUSTOM_CODE_LEN))
}