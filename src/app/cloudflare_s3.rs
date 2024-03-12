use std::env;
use rusoto_core::Region; // v0.44.0
use rusoto_credential::StaticProvider; // v0.44.0
use rusoto_s3::S3Client;

use crate::library::constant;

#[derive(Clone)]
pub struct CloudflareS3{
    pub s3_client: S3Client
}

impl CloudflareS3 {
    
    pub fn init()-> Self{
        let api_url = env::var("CLOUDFLARE_S3_API").expect("CLOUDFLARE_S3_API not found");
        let access_key = env::var("CLOUDFLARE_S3_ACCESS_KEY_ID").expect("CLOUDFLARE_S3_ACCESS_KEY_ID not found");
        let secret_key = env::var("CLOUDFLARE_S3_SECRET_ACCESS_KEY").expect("CLOUDFLARE_S3_SECRET_ACCESS_KEY not found");

        let region = Region::Custom {
            name: constant::CLOUDFLARE_REGION.to_owned(), // Give it a unique name
            endpoint: api_url, // Specify your custom endpoint
        };
    
        // Initialize the S3 client with the custom endpoint
        let s3_client = S3Client::new_with(
            rusoto_core::request::HttpClient::new().unwrap(),
            StaticProvider::new_minimal(access_key, secret_key),
            region,
        );

        CloudflareS3{ s3_client }
    }
}
