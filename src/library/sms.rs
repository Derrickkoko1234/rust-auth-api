// sending sms with lambahq
use std::env;
use reqwest::header::{HeaderMap, HeaderValue};
use mongodb::error::Error;

use super::{constant, schema::SMSConfig};

pub async fn send_sms(sms_config: &SMSConfig)->Result<bool,Error>{
    let lamba_user = env::var("LAMBA_CUSTOMER_ID").expect("LAMBA_CUSTOMER_ID not found");

    let url = format!("{}/action/{}",constant::LAMBA_API,lamba_user);

    let lamba_key = env::var("LAMBA_API_KEY").expect("LAMBA_API_KEY not found");

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(format!("Basic {}",lamba_key.as_str()).as_str()).unwrap());

    // Create a reqwest client with custom headers
    let client = reqwest::Client::builder().default_headers(headers).build().unwrap();

    let json_value = serde_json::to_value(sms_config).unwrap();

    // Send the POST request with the custom headers
    let response = client
    .post(url)
    .json(&json_value)
    .send()
    .await;

    if let Err(e) = response{
        return Err(Error::custom(format!("Error: {:?}",e)));
    }

    let is_success = response.unwrap().status().is_success();
    Ok(is_success)
}