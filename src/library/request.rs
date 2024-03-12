
use std::fmt::Debug;
use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderValue,HeaderName};
use reqwest::{Client, Response, Error};
use serde::Deserialize;

pub async fn make_post_request<T: for<'a> Deserialize<'a> + Debug>(url:&str,params: &[(&str,&str)],headers: Option<Vec<(String,String)>>) -> Result<T, Error> {
    let client = Client::new();
    let response:Response;

    match headers{
        Some(hdrs) => {
            let mut headermap = HeaderMap::new();
            for (k,v) in hdrs{
                headermap.insert(HeaderName::from_str(&k).unwrap(),HeaderValue::from_str(&v).unwrap());
            }

            if params.len().gt(&0){
                response = client.post(url).headers(headermap).form(params).send().await?;
            }else{
                response = client.post(url).headers(headermap).send().await?;
            }
        },
        None => {
            if params.len().gt(&0){
                response = client.post(url).form(params).send().await?;
            }else{
                response = client.post(url).send().await?;
            }
        },
    }

    let r = response.json::<T>().await?;
    Ok(r)
}

pub async fn make_get_request<T: for<'a> Deserialize<'a>>(url: &str,headers: Option<Vec<(String,String)>>) -> Result<T, Error> {
    let client = Client::new();
    let response:Response;

    match headers{
        Some(hdrs) => {
            let mut headermap = HeaderMap::new();
            for (k,v) in hdrs{
                headermap.insert(HeaderName::from_str(&k).unwrap(),HeaderValue::from_str(&v).unwrap());
            }

            response = client.get(url).headers(headermap).send().await?;
        },
        None => {
            response = client.get(url).send().await?;
        },
    }

    let r = response.json::<T>().await?;
    Ok(r)
}
