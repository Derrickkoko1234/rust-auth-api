extern crate redis;
use std::env;
use redis::{
    Client, RedisError, Commands,
};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::library::constant;

#[derive(Debug, Clone)]
pub struct Redis{
    pub client: Client
}

impl Redis {
    
    pub fn init()-> Result<Self,RedisError>{
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL not found");

        let client = redis::Client::open(redis_url)?;
        Ok(Redis { client })
    }

    // get helpers
    pub fn get_string_item(client: &mut Client,key: &str)->String{
        let mut conn = client.get_connection().unwrap();
        let result: Result<String, RedisError> = conn.get(key);
        match &result {
            Ok(v) => v.to_string(),
            Err(_) => constant::EMPTY_STR.to_owned(),
        }
    }

    pub fn get_json_item<T: for<'a> Deserialize<'a>>(client: &mut Client,key: &str)->Option<T>{
        let mut conn = client.get_connection().unwrap();
        let data: Result<String, RedisError> = conn.get(key);
        match &data {
            Ok(v) => {
                let deserialized: T = serde_json::from_str(&v).unwrap();
                Some(deserialized)
            },
            Err(_) => None,
        }
    }

    // set helpers
    pub fn set_string_item(client: &mut Client,key: &str,value: &str,expiration: usize){
        let mut conn = client.get_connection().unwrap();
        let _: () = conn.set(key, value).unwrap();
        let _: ()  = conn.expire(key, expiration as i64).unwrap();
    }

    pub fn set_json_item<U: Serialize>(client: &mut Client,key: &str,value: &U,expiration: usize){
        let mut conn = client.get_connection().unwrap();
        let serialized: String = serde_json::to_string(value).unwrap();
        let _: () = conn.set(key, &serialized).unwrap();
        let _: () = conn.expire(key, expiration as i64).unwrap();
    }

    // get matching keys helpers.
    pub fn get_keys(client: &mut Client,key: &str)->Vec<String>{
        let mut conn = client.get_connection().unwrap();
        let keys = conn.scan_match(format!("*{}*",key)).unwrap();
        let keys: Vec<String> = keys.into_iter().collect();

        keys
    }

    // delete helpers.
    pub fn delete_item(client: &mut Client,key: &str){
        let mut conn = client.get_connection().unwrap();
        let _: () = conn.del(key).unwrap();

        // delete by keys too if it exists
        let keys = Redis::get_keys(client, key);
        if keys.len() > 0 {
            for k in keys {
                let _: () = conn.del(k).unwrap();
            }
        }
    }
}