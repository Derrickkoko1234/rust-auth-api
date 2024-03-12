// sending email with lambahq
use std::env;
use reqwest::header::{HeaderMap, HeaderValue};
use html_minifier::minify;
use mongodb::error::Error;
use crate::library::{constant, file};

use super::schema::{EmailConfig, TemplateBuilder};

pub async fn send_email(email_config: &EmailConfig)->Result<bool,Error>{
    let lamba_user = env::var("LAMBA_CUSTOMER_ID").expect("LAMBA_CUSTOMER_ID not found");

    let url = format!("{}/action/{}",constant::LAMBA_API,lamba_user);

    let lamba_key = env::var("LAMBA_API_KEY").expect("LAMBA_API_KEY not found");

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(format!("Basic {}",lamba_key.as_str()).as_str()).unwrap());

    // Create a reqwest client with custom headers
    let client = reqwest::Client::builder().default_headers(headers).build().unwrap();

    let json_value = serde_json::to_value(email_config).unwrap();

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

pub fn build_template(builder: &TemplateBuilder)->Result<String,Error>{
    // get file content
    let file_content = file::read_file_content_as_string(format!("{}{}.html",constant::EMAILTEMPLATE_PATH,builder.template))?;

    // get footer content
    let footer_content = file::read_file_content_as_string(format!("{}{}.html",constant::EMAILTEMPLATE_PATH,constant::FOOTER))?;
    
    let mut email_template = format!("{} {}",file_content,footer_content);
    for replacer in builder.replacers.clone(){
        let rpl = format!("%{}%",replacer.0);
        email_template = email_template.replace(&rpl, &replacer.1);
    }
   
    let minified_contents = minify(email_template);
    if let Err(e) = minified_contents{
        return Err(Error::custom(format!("Error: {:?}",e)));
    }
    Ok(minified_contents.unwrap())
}

pub fn build_template_without_footer(builder: &TemplateBuilder)->Result<String,Error>{
    // get file content
    let file_content = file::read_file_content_as_string(format!("{}{}.html",constant::EMAILTEMPLATE_PATH,builder.template))?;

    let mut email_template = file_content;
    for replacer in builder.replacers.clone(){
        let rpl = format!("%{}%",replacer.0);
        email_template = email_template.replace(&rpl, &replacer.1);
    }
   
    let minified_contents = minify(email_template);
    if let Err(e) = minified_contents{
        return Err(Error::custom(format!("Error: {:?}",e)));
    }
    Ok(minified_contents.unwrap())
}