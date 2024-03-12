use mongodb::error::Error;
use serde::{Serialize,Deserialize};
use validator::ValidationErrors;

use crate::library::constant;

#[derive(Clone,Serialize,Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T
}

impl<T: 'static> APIResponse<T> {
    pub fn new(success:bool,message:String,data:T)->Self{
        Self{
            success,
            message,
            data
        }
    }
}

pub fn get_custom_err_msg(e: &str)->APIResponse<serde_json::Value>{
    let ext= format!("Reason: {:?}",e);
    APIResponse::new(false, ext, serde_json::Value::Null)
}

pub fn get_custom_success_msg(e: &str)->APIResponse<serde_json::Value>{
    let ext= format!("Reason: {:?}",e);
    APIResponse::new(true, ext, serde_json::Value::Null)
}

pub fn get_success_msg<T: Serialize>(payload:T)->APIResponse<serde_json::Value>{
    let data = serde_json::to_value(&payload).unwrap();
    APIResponse::new(true, constant::SUCCESS.to_owned(), data)
}

pub fn _get_err_msg(e: Option<Error>)->APIResponse<serde_json::Value>{
    let ext;
    if let Some(e) = e{
        ext = format!("Reason: {:?}",e);
    }else{
        ext = constant::NOT_FOUND.to_owned();
    }
    APIResponse::new(false, ext, serde_json::Value::Null)
}

pub fn get_validation_err_msg(e:ValidationErrors)->APIResponse<serde_json::Value>{
    let ext = format!("Ivalid `{}`! Please review your input and try again",e.errors().into_iter().next().unwrap().0);
    APIResponse::new(false, ext, serde_json::Value::Null)
}